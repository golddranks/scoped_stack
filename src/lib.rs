#[derive(Clone, Eq, PartialEq)]
pub struct Stack<'a, T: 'a> {
	frame: Option<StackFrame<'a, T>>,
}

#[derive(Clone, Eq, PartialEq)]
struct StackFrame<'a, T: 'a> {
	parent: &'a Stack<'a, T>,
	value: T,
}

#[derive(Clone, Eq, PartialEq)]
pub struct StackIter<'a, T: 'a> {
	current: &'a Stack<'a, T>,
}

#[derive(Clone, Eq, PartialEq)]
pub struct StackFrameIter<'a, T: 'a> {
	current: &'a Stack<'a, T>,
}

impl<'a, T> Stack<'a, T> {
	#[must_use]
	pub fn push<'b>(&'b self, value: T) -> Stack<'b, T> {
		Stack {
			frame: Some(StackFrame {
				parent: &self,
				value,
			})
		}
	}

	#[must_use]
	pub fn new() -> Stack<'a, T> {
		Stack {
			frame: None,
		}
	}

	#[must_use]
	pub fn pop(self) -> Option<T> {
		self.frame.map(|frame| frame.value)
	}

	pub fn peek(&self) -> Option<&T> {
		self.frame.as_ref().map(|frame| &frame.value)
	}

	pub fn iter<'s>(&'s self) -> StackIter<'s, T> {
		StackIter { current: self }
	}

	pub fn iter_frames<'s>(&'s self) -> StackFrameIter<'s, T> {
		StackFrameIter { current: self }
	}
}

impl<'a, T> std::fmt::Debug for Stack<'a, T> where T: std::fmt::Debug {
	fn fmt(&self, formatter: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
		formatter.write_str("[")?;
		formatter.write_str("<top>, ")?;
		for item in self.iter() {
			item.fmt(formatter)?;
			formatter.write_str(", ")?;
		}
		formatter.write_str("<base>")?;
		formatter.write_str("]")?;
		Ok(())
	}
}

impl<'a, T> Iterator for StackIter<'a, T> {
	type Item = &'a T;

	fn next(&mut self) -> Option<&'a T> {
		self.current.frame.as_ref().map(|frame| {
			self.current = frame.parent;
			&frame.value
		})
	}
}

impl<'a, T> Iterator for StackFrameIter<'a, T> {
	type Item = &'a Stack<'a, T>;

	fn next(&mut self) -> Option<&'a Stack<'a, T>> {
		self.current.frame.as_ref().map(|frame| {
			let item = self.current;
			self.current = frame.parent;
			item
		})
	}
}

#[cfg(test)]
mod tests {
	use Stack;

    #[test]
    fn it_works() {
        let stack = Stack::new();

        println!("{:?}", stack);

        let a = stack.push("hoge a");
        let b = stack.push("hoge b");

        println!("{:?}", a);
        println!("{:?}", b);

        let aa = a.push("fuga aa");
        let ab = a.push("fuga ab");
        let ba = b.push("fuga ba");
        let bb = b.push("fuga bb");

        println!("{:?}", aa);
        println!("{:?}", ab);
        println!("{:?}", ba);
        println!("{:?}", bb);
    }
}