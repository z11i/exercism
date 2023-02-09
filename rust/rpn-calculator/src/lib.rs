#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut stack: Vec<i32> = Vec::new();
    for input in inputs {
        match input {
            CalculatorInput::Value(v) => stack.push(*v),
            _ => {
                let a = stack.pop()?;
                let b = stack.pop()?;
                let result = match input {
                    CalculatorInput::Add => b + a,
                    CalculatorInput::Subtract => b - a,
                    CalculatorInput::Multiply => b * a,
                    CalculatorInput::Divide => b / a,
                    CalculatorInput::Value(_) => todo!(),
                };
                stack.push(result);
            }
        }
    }
    if stack.len() != 1 {
        return None;
    }
    return stack.pop();
}
