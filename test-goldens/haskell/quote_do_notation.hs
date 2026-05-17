main :: IO ()
main = do
  putStrLn "Enter name:"
  name <- getLine
  putStrLn ("Hello, " ++ name)
