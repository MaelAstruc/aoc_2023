library(stringr)

day_1 <- function(string) {
  # Keep first and last letters to handle overlap situations
  numbers <- c(
    "one" = "o1e",
    "two" = "t2o",
    "three" = "t3e",
    "four" = "f4r",
    "five" = "f5e",
    "six" = "s6x",
    "seven" = "s7n",
    "eight" = "e8t",
    "nine" = "n9e"
  )

  values <- string |>
    str_replace_all(numbers) |> # Use names as patterns and replace by values
    str_remove_all("[^\\d]") |> # Remove non digits
    str_remove_all("(?<=\\d).*(?=\\d)") |> # Remove digits in the middle
    as.numeric()

  # Handle lonely digits by adding * 10 if below 10
  values <- values + values * 10 * (values < 10)

  sum(values)
}

part_1 <- c(
  "1abc2",
  "pqr3stu8vwx",
  "a1b2c3d4e5f",
  "treb7uchet"
)

part_2 <- c(
  "two1nine"
  ,"eightwothree"
  ,"abcone2threexyz"
  ,"xtwone3four"
  ,"4nineeightseven2"
  ,"zoneight234"
  ,"7pqrstsixteen"
)

day_1(part_1) # 42
day_1(part_2) # 281
