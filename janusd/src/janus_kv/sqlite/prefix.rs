// SPDX-License-Identifier: GPL-3.0-or-later

//! Successor bound for byte-prefix range scans.

/// Successor bound of `prefix`: increments the last byte below 0xFF and
/// truncates; all-0xFF yields `None` (no upper bound).
pub(crate) fn prefix_upper(prefix: &[u8]) -> Option<Vec<u8>> {
	let mut bound = prefix.to_vec();
	while let Some(last) = bound.last_mut() {
		if *last < 0xFF {
			*last += 1;
			return Some(bound);
		}
		bound.pop();
	}
	None
}

#[cfg(test)]
mod tests {
	use super::prefix_upper;

	#[test]
	fn increments_last_byte() {
		assert_eq!(prefix_upper(&[1, 2, 3]), Some(vec![1, 2, 4]));
		assert_eq!(prefix_upper(&[0xFE]), Some(vec![0xFF]));
	}

	#[test]
	fn carries_past_0xff() {
		assert_eq!(prefix_upper(&[1, 0xFF]), Some(vec![2]));
		assert_eq!(prefix_upper(&[1, 0xFE, 0xFF]), Some(vec![1, 0xFF]));
	}

	#[test]
	fn all_0xff_has_no_bound() {
		assert_eq!(prefix_upper(&[0xFF, 0xFF]), None);
	}
}
