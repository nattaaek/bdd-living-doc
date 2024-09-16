Feature: User Login

  As a registered user
  I want to log into the system
  So that I can access protected areas

  Scenario: Successful login with valid credentials
    Given I am on the login page
    When I enter valid credentials
    Then I should be redirected to the dashboard

  Scenario: Unsuccessful login with invalid credentials
    Given I am on the login page
    When I enter invalid credentials
    Then I should see an error message
