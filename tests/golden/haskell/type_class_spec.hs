-- | Things that can be printed.
class Printable where
  prettyPrint :: a -> String
