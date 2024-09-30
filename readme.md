# Overview

## Card
-- 52 cards
-- Number cards have their own value
-- Ace has value 1 or 11
-- Jack, Queen, King all have value of 10

## Players
-- There will be two players, the dealer and the player/user

## Win
-- Player wins who is closest to 21 without going over
-- A hand that includes an Ace and 10 is called a "Blackjack". It pays 3 to 2
-- If the dealer also has a Blackjack, it's a "push" and neither player wins
-- If a player's total is nearer 21 than the Dealer's, they win even money

## Play
-- A player can "stand" if they are satisfied with their two cards'
-- A player can "hit" to draw one or more cards
-- The dealer will hit until it has 17 or more, regardless of what the players have. Unless the players have busted, then the dealer does not hit.
  -- Some casinos allow a dealer to hit on a soft 17 (i.e., an A and a 6)
-- Game begins with dealer handing out two cards to each player, face down
-- players can see own cards
-- player is given the option to hit or stand
-- if stand: play moves to the dealer to hit until at least 17
-- if hit: new card is shown, if over 21, bust and player loses. If under 21, player gets option to hit or stand.

# Code

## Game
-- Game has a start. Game start begins with Do you want to play: "Yes". Then dealing the cards.
-- Game has rounds. A round ends after a hit/stand
-- Game has an end.

## Players
-- Players have: name, turn (a turn is similar to a game->round)
-- Version 2 players can have money

## Cards
-- Cards need to have a name and value

## Rules
-- 52 cards in deck
-- max sum a player can hold is 21
-- player with largest sum under 21 wins
-- A dealt Ace / 10 is auto blackjack
