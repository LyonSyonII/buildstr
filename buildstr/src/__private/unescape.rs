#![warn(missing_docs)]

// Code from https://github.com/hack-ink/unescaper under the GPL-3 license.
// Changes made:
// - Remove use of `thiserror`, instead implementing the `Error` trait on our own.
// - Changed `Error` to `UnescapeError` and added explicit enum variants (instead of using `Error::*`).

//! Unescape the given string.
//! This is the opposite operation of [`::std::ascii::escape_default`].

pub type Result<T> = ::std::result::Result<T, UnescapeError>;

#[derive(Debug)]
pub enum UnescapeError {
	IncompleteStr(usize),
	InvalidChar { char: char, pos: usize },
	ParseIntError { source: ::std::num::ParseIntError, pos: usize },
}

impl ::std::fmt::Display for UnescapeError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            UnescapeError::IncompleteStr(s) => write!(f, "incomplete str, break at {s}"),
            UnescapeError::InvalidChar { char, pos } => write!(f, "invalid char, {char:?} break at {pos}"),
            UnescapeError::ParseIntError { pos, .. } => write!(f, "parse int error, break at {pos}"),
        }
    }
}

impl ::std::error::Error for UnescapeError {}

/// Unescaper struct that holds the chars cache for unescaping.
#[derive(Debug)]
pub struct Unescaper {
	/// [`str`] cache, in reverse order.
	pub chars: Vec<char>,
}
impl Unescaper {
	/// Build a new [`Unescaper`] from the given [`str`].
	pub fn new(s: &str) -> Self {
		Self { chars: s.chars().rev().collect() }
	}

	/// Unescape the given [`str`].
	pub fn unescape(&mut self) -> Result<String> {
		let chars_count = self.chars.len();
		let offset = |mut e, remaining_count| {
			let (UnescapeError::IncompleteStr(pos) | UnescapeError::InvalidChar { pos, .. } | UnescapeError::ParseIntError { pos, .. }) = &mut e;

			*pos += chars_count - remaining_count - 1;

			e
		};
		let mut unescaped = String::new();

		while let Some(c) = self.chars.pop() {
			if c != '\\' {
				unescaped.push(c);

				continue;
			}

			let c = self.chars.pop().ok_or(UnescapeError::IncompleteStr(chars_count - self.chars.len() - 1))?;
			let c = match c {
				'b' => '\u{0008}',
				'f' => '\u{000c}',
				'n' => '\n',
				'r' => '\r',
				't' => '\t',
				// https://github.com/hack-ink/unescaper/pull/10#issuecomment-1676443635
				//
				// https://www.ecma-international.org/wp-content/uploads/ECMA-404_2nd_edition_december_2017.pdf
				// On page 4 it says: "\/ represents the solidus character (U+002F)."
				'\'' | '\"' | '\\' | '/' => c,
				'u' => self.unescape_unicode_internal().map_err(|e| offset(e, self.chars.len()))?,
				'x' => self.unescape_byte_internal().map_err(|e| offset(e, self.chars.len()))?,
				_ => self.unescape_octal_internal(c).map_err(|e| offset(e, self.chars.len()))?,
			};

			unescaped.push(c);
		}

		Ok(unescaped)
	}

	// pub fn unescape_unicode(&mut self) -> Result<char> {}
	fn unescape_unicode_internal(&mut self) -> Result<char> {
		let c = self.chars.pop().ok_or(UnescapeError::IncompleteStr(0))?;
		let mut unicode = String::new();

		// \u + { + regex(d*) + }
		if c == '{' {
			while let Some(n) = self.chars.pop() {
				if n == '}' {
					break;
				}

				unicode.push(n);
			}
		}
		// \u + regex(d{4})
		else {
			// [0, 65536), 16^4
			unicode.push(c);

			for i in 0..3 {
				let c = self.chars.pop().ok_or(UnescapeError::IncompleteStr(i))?;

				unicode.push(c);
			}
		}

		char::from_u32(
			u16::from_str_radix(&unicode, 16).map_err(|e| UnescapeError::ParseIntError { source: e, pos: 0 })?
				as _,
		)
		.ok_or(UnescapeError::InvalidChar {
			char: unicode.chars().last().expect("empty unicode will exit earlier; qed"),
			pos: 0,
		})
	}

	// pub fn unescape_byte(&mut self) -> Result<char> {}
	fn unescape_byte_internal(&mut self) -> Result<char> {
		let mut byte = String::new();

		// [0, 256), 16^2
		for i in 0..2 {
			let c = self.chars.pop().ok_or(UnescapeError::IncompleteStr(i))?;

			byte.push(c);
		}

		Ok(u8::from_str_radix(&byte, 16).map_err(|e| UnescapeError::ParseIntError { source: e, pos: 0 })? as _)
	}

	// pub fn unescape_octal(&mut self) -> Result<char> {}
	fn unescape_octal_internal(&mut self, c: char) -> Result<char> {
		let mut octal = String::new();
		let mut try_push_next = |octal: &mut String| {
			if let Some(c) =
				self.chars.last().cloned().filter(|c| c.is_digit(8)).and_then(|_| self.chars.pop())
			{
				octal.push(c);
			}
		};

		match c {
			// decimal [0, 256) == octal [0, 400)
			// 0 <= first digit < 4
			// \ + regex(d{1,3})
			'0' | '1' | '2' | '3' => {
				octal.push(c);

				(0..2).for_each(|_| try_push_next(&mut octal));
			},
			// \ + regex(d{1,2})
			'4' | '5' | '6' | '7' => {
				octal.push(c);

				try_push_next(&mut octal);
			},
			_ => Err(UnescapeError::InvalidChar { char: c, pos: 0 })?,
		}

		Ok(u8::from_str_radix(&octal, 8).map_err(|e| UnescapeError::ParseIntError { source: e, pos: 0 })? as _)
	}
}

/// Unescape the given [`str`].
pub fn unescape(s: &str) -> Result<String> {
	Unescaper::new(s).unescape()
}