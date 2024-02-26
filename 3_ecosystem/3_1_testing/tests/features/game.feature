Feature: Game

  Scenario: If we pass in same number
    Given a program is running
    When player passes string: 12
    And player passes string: 12
    Then program works as intended


  Scenario: If we play the game to win
    Given a program is running
    When player passes string: 12
    And player passes string: 101
    And player passes winning number
    Then program works as intended
  
  Scenario: Player wins a game from fisrt try
    Given a program is running
    When player passes winning number
    Then program works as intended

  Scenario: Player passes in a bad input 
    Given a program is running
    When player passes string: boo
    Then program works as intended

  Scenario: Player guesses on second try
    Given a program is running
    When player passes string: 12
    And player passes winning number
    # Then program works as intended
    Then we win

  Scenario: Player guesses after wrong input
    Given a program is running
    When player passes string: foo
    And player passes winning number
    # Then program works as intended
    Then we win