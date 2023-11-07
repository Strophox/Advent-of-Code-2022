day = "21"

main = do
  txt <- init <$> readFile (day<>".txt")
  let blueprints = map (fmap (tail . words) . break (==':')) (lines txt)
  let tree = buildTree blueprints
  let evaluate = foldTree (const parseOp) (const id)
  let sol1 = evaluate tree
  donePart 1 sol1
  let sol2 =  evaluate ("humn"`rebalanceTree`tree) -- ooga booga >:O
  donePart 2 sol2
  where
    donePart i sol = putStrLn $ "The solution of day "<>day<>" part "<>show i<>" is: "<>show sol

infix 5 !>
(!>) :: (Eq a)=> [(a,b)] -> a -> b
keyValueMap !> x = maybe (error "key not found") id (lookup x keyValueMap)

opStrings   = ["+", "-", "*", "/"]
opSemantics = [(+), (-), (*), div]
opInverses  = ["-", "+", "/", "*"]
commutativeOps = ["+","*"]

parseOp = (opStrings`zip`opSemantics !>)
invOp   = (opStrings`zip`opInverses !>)

data Tree l o a = Leaf l a | Node l o (Tree l o a) (Tree l o a)
-- instance Functor (Tree l o) where
--   fmap f (Leaf l a) = Leaf l (f a)
--   fmap f (Node l o t1 t2) = Node l o (fmap f t1) (fmap f t2)
foldTree :: (l -> o -> b -> b -> b) -> (l -> a -> b) -> Tree l o a -> b
foldTree opNode opLeaf = fold
  where fold (Leaf l a) = opLeaf l a
        fold (Node l op t1 t2) = opNode l op (fold t1) (fold t2)

type UsualTree = Tree String String Int

buildTree :: [(String, [String])] -> UsualTree
buildTree rawExprs = sprout "root"
  where sprout name = case rawExprs !> name of
          [val] -> Leaf name (read val)
          [name1,op,name2] -> Node name op (sprout name1) (sprout name2)

rebalanceTree :: String -> UsualTree -> UsualTree
rebalanceTree bias rt@(Node root _ l r)
  | hasSolVar rt = balance (Node root "-" l r) (Leaf "" 0)
  | otherwise    = error "variable to solve for not found"
  where
    hasSolVar = foldTree (\x _ l r -> x==bias || l || r) (\x _ -> x==bias)
    balance :: UsualTree -> UsualTree -> UsualTree
    balance (Leaf _ _) other = other
    balance (Node name op left right) other
      | name == bias    = other
      | hasSolVar left  = balance left (Node name (invOp op) other right)
      | hasSolVar right = if op`elem`commutativeOps
                          then balance right (Node name (invOp op) other left)
                          else balance (Node name (invOp op) other right) left

{- part 1 old
main = do
  txt <- init <$> readFile (day<>".txt")
  let blueprints = map (fmap (tail . words) . break (==':')) (lines txt)
  let expressions = evalExpressions blueprints
  let sol1 = expressions !> "root"
  donePart 1 sol1
  let sol2 = "todo"
  donePart 2 sol2
  where
    donePart i sol = putStrLn $ "The solution of day "<>day<>" part "<>show i<>" is: "<>show sol

evalExpressions :: [(String, [String])] -> [(String, Int)]
evalExpressions blueprints = exprs
  where
    exprs = map (fmap evaluate) blueprints
    ops = [("+",(+)), ("-",(-)), ("*",(*)), ("/",div)]
    evaluate [n0] = read n0
    evaluate [n1,op,n2] = (ops !> op) (exprs !> n1) (exprs !> n2)

infix 5 !>
(!>) :: (Eq a)=> [(a,b)] -> a -> b
keyValueMap !> x = maybe (error "key not found") id (lookup x keyValueMap)
-}
