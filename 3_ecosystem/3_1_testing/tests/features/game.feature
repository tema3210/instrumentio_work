Feature: Same number doesn't recieve random input

  Scenario: If we pass in same number
    Given a program is running
    When we pass in "3" "2" times
    Then program produces same output
  
  Scenario: Try to win a game 
    Given a program is running
    When we guess a number right
    Then we win

  Scenario: If we pass in a bad input 
    Given a program is running
    When we pass not a number input
    Then program ignores a line