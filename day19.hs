day = "19" -- https://adventofcode.com/2022/day/19

main = do
  txt <- init <$> readFile (day<>".txt")
  let sol1 = undefined
  donePart 1 sol1
  let sol2 = undefined
  donePart 2 sol2
  where
    donePart i sol = putStrLn $ "The solution of day "<>day<>" part "<>show i<>" is: "<>show sol
