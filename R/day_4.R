library(stringr)

input <- c(
  "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53",
  "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19",
  "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1",
  "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83",
  "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36",
  "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"
)

day4 <- function(input) {
  data <- data.frame(input = input)
  
  data <- data |>
    mutate(
      input_clean = str_remove(input, ".*:"),
      input_clean = str_trim(input_clean),
      input_clean = str_replace_all(input_clean, "[ ]+", " "),
    )
  
  data[c("winning", "have")] <- str_split(data$input_clean, "\\|", simplify = TRUE)
  
  data[["winning"]] <- lapply(data$winning, clean_num_list)
  data[["have"]] <-  lapply(data$have, clean_num_list)
  
  data[["n"]] <- apply(data, 1, function(x) sum(x$have %in% x$winning))
  
  data[["points"]] <- if_else(data$n == 0, 0, 2^(data$n - 1))
  
  data[["copies"]] <- 1
  
  for (i in 1:nrow(data)) {
    n_i <- data[i, "n"]
    if (n_i > 0) {
      n_i <- min(nrow(data), n_i)
      range <- (i+1):(i+n_i)
      data[range, "copies"] <- data[range, "copies"] + data[i, "copies"]
    }
  }
  
  print(sum(data$points))
  print(sum(data$copies))
}

clean_num_list <- function(number_string) {
  number_string |>
    str_trim() |>
    str_split(" ", simplify = TRUE) |>
    as.numeric()
}

day4(input)

