Feature: Game

  Scenario: If we pass in same number
    Given a program is running
    When we pass string: 12
    And we pass string: 12
    Then program produces sane output


  Scenario: If we play the game to win
    Given a program is running
    When we pass string: 12
    And we pass string: 101
    And we pass winning number
    Then program produces sane output
  
  Scenario: Try to win a game 
    Given a program is running
    When we pass winning number
    Then we win

  Scenario: If we pass in a bad input 
    Given a program is running
    When we pass string: boo
    Then program ignores a line