Feature: Purchase Items

  In order to buy items
  As a customer
  I want to add items to my cart and purchase them

  Scenario: Add item to cart
    Given I am on the items page
    When I add an item to the cart
    Then the item should appear in my cart

  Scenario: Checkout cart
    Given I have items in my cart
    When I proceed to checkout
    Then I should be able to complete the purchase
