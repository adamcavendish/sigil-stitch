abs :: Int -> Int
abs n
  | n < 0 = negate n
  | otherwise = n
