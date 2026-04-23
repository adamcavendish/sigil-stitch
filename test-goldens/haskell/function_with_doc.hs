-- | Greet the user by name.
greet :: String -> IO ()
greet name =
  putStrLn ("Hello, " ++ name)
