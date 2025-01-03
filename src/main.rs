// Basicamente, na arvore binaria a gente tem uma regra
// Valor < Valor atual = inserir na esquerda
// Valor > Valor atual = inserir na direita
struct TreeNode<T> {
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>
}

// Nesse caso, é extremamente importante o "&". Com eles, a gente nao copia o dado pro contexto da
// função, simplesmente passamos uma referencia imutavel para verificar o valor e protinho.
// Economizamos memoria
fn search_value_on_node<T: PartialOrd>(node: &Option<Box<TreeNode<T>>>, search_value: T) -> bool {
        return match node {
            Some(node) => node.contains(search_value),
            None => false,
        };
}

impl<T: PartialOrd> TreeNode<T> {
    fn new(value: T) -> Self {
        TreeNode { value, left: None, right: None }
    } 
    fn insert(&mut self, new_value: T) {
        if new_value < self.value {
            if let Some(ref mut left) = self.left {
                left.insert(new_value);
                return;
            }
            self.left = Some(Box::new(TreeNode::new(new_value)));
            return;
        }
        if let Some(ref mut right) = self.right {
            right.insert(new_value);
            return;
        }
        self.right = Some(Box::new(TreeNode::new(new_value)));
        return;
    }
    fn contains(&self, search_value: T) -> bool {
        // primeiro nivel
        if search_value == self.value {
            return true;
        }
        if search_value < self.value {
            return search_value_on_node(&self.left, search_value);
        } else {
            return search_value_on_node(&self.right, search_value);
        }
    }
}


#[cfg(test)]
mod tests {
    use super::TreeNode;

    #[test]
    fn test_new_tree() {
        let tree = TreeNode::new(10);

        assert_eq!(tree.value, 10);
        assert!(tree.left.is_none());
        assert!(tree.right.is_none());
    }
    #[test]
    fn test_insert_left() {
        // em rust esse mut se torna necessario pois estamso modificando a estrutura de dados
        // apos a sua criação
        let mut tree = TreeNode::new(10);
        tree.insert(9);

        assert!(tree.left.is_some());
        // as_ref serve para acessar a referencia, enquanto o unwrap serve para acessarmos o valor
        // do option considerando que não sera um None
        assert_eq!(tree.left.as_ref().unwrap().value, 9);
    }
    #[test]
    fn test_insert_right() {
        let mut tree = TreeNode::new(10);
        tree.insert(11);

        assert!(tree.right.is_some());
        assert_eq!(tree.right.as_ref().unwrap().value, 11);
    }
    #[test]
    fn test_insert_multiple_levels() {
        let mut tree = TreeNode::new(10);
        tree.insert(5);
        tree.insert(15);
        tree.insert(3);
        tree.insert(7);
        tree.insert(12);
        tree.insert(18);

        // Preciso usar o as_ref e unwrap nos subniveis tbm
        assert_eq!(tree.left.as_ref().unwrap().left.as_ref().unwrap().value, 3);
        assert_eq!(tree.left.as_ref().unwrap().right.as_ref().unwrap().value, 7);
        assert_eq!(tree.right.as_ref().unwrap().left.as_ref().unwrap().value, 12);
        assert_eq!(tree.right.as_ref().unwrap().right.as_ref().unwrap().value, 18);
    }
    #[test]
    fn test_contains_value() {
        let mut tree = TreeNode::new(10);
        tree.insert(5);
        tree.insert(15);

        assert!(tree.contains(10));
        assert!(tree.contains(5));
        assert!(tree.contains(15));
        assert!(!tree.contains(8));
        assert!(!tree.contains(20));
    }
}


fn main() {}
