use std::error::Error;

#[derive(Debug, PartialEq)]
enum Action {
    Read,
    Write,
}

#[derive(Debug, PartialEq)]
pub struct Block {
    pub node_id: usize,
    action: Action,
}

impl Block {
    pub fn is_conflict(&self, other: &Self) -> bool {
        self.node_id != other.node_id
            && (self.action == Action::Write || other.action == Action::Write)
    }
}

fn parse_block(input: &str) -> Result<Block, Box<dyn Error>> {
    let action = match input.chars().next() {
        Some('r') => Action::Read,
        Some('w') => Action::Write,
        other => {
            return Err(
                format!("'{}' is not a valid action char", other.unwrap_or_default()).into(),
            );
        }
    };

    let node_id: usize = input[1..].parse::<usize>()?;

    Ok(Block { node_id, action })
}

pub fn parse_blocks(input: &str) -> Result<Vec<Block>, Box<dyn Error>> {
    let block_list = {
        let input: Vec<&str> = input.trim().split(",").collect();
        let mut v = vec![];
        for b in input {
            if !b.is_empty() {
                v.push(parse_block(b)?);
            }
        }
        v
    };

    if block_list.is_empty() {
        Err("The schedule doesn't have any actions".into())
    } else {
        Ok(block_list)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_block() -> Result<(), Box<dyn Error>> {
        assert_eq!(
            parse_block("w127")?,
            Block {
                node_id: 127,
                action: Action::Write,
            }
        );
        Ok(())
    }

    #[test]
    fn write_block() {
        let err = parse_blocks(" ").unwrap_err();
        assert_eq!(err.to_string(), "The schedule doesn't have any actions");
    }

    #[test]
    fn read_block() -> Result<(), Box<dyn Error>> {
        assert_eq!(
            parse_block("r59")?,
            Block {
                node_id: 59,
                action: Action::Read,
            }
        );
        Ok(())
    }

    #[test]
    fn invalid_block_action() {
        let err = parse_block("f59").unwrap_err();
        assert_eq!(err.to_string(), "'f' is not a valid action char");
    }

    #[test]
    fn invalid_block_node_id() {
        let err = parse_block("rabc").unwrap_err();
        assert!(err.is::<std::num::ParseIntError>());
    }

    #[test]
    fn test_find_nodes() -> Result<(), Box<dyn Error>> {
        assert_eq!(
            parse_blocks("w1,r2")?,
            vec![
                Block {
                    node_id: 1,
                    action: Action::Write
                },
                Block {
                    node_id: 2,
                    action: Action::Read
                }
            ]
        );
        Ok(())
    }
}
