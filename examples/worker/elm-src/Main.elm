module Main exposing (..)

import List exposing (repeat, map)
import Platform.Cmd exposing (none)


{- this is
   {-- a-}
  {-nested--t}}}}----{}{{{{{}}}}}
    {-comment-}
  -}
-}


num : Int
num =
    -- comment
    .a { a = 3 + 2, b = 2.1, c =
          "some \n \"text\\\""
           ++ "text"
     }
