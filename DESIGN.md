Design notes
===

User Stories
---

Adapted from the previous python+FastAPI version of the API-only implementation.

### MVP:

1. When a User navigates to `/register`, they are shown a form to create a new user account
  1. When a User submits the form successfully, the new user account is created & they are taken to `/onboarding`
  2. When a User attempts to submit the form and their is an error, they stay on the form with the last information they attempted to submit and an error message is rendered at the top (if a particular form field is responsible for the error, it is highlighted as well)

1. When a new User navigates to `/onboarding` and they **haven't** completed any steps, are taken to `/onboarding/1`
2. When a User navigates to `/onboarding` and they **have** completed any steps, they are taken to the first step they haven't completed yet&mdash;if they have completed all steps, they are instead taken to `/dashboard`

1. When a User navigates to `/onboarding/1`, they are shown first shown a form asking them to enter the information for their first Account
  1. When a User submits the form successfully, the new Account is saved & they are shown a modal asking if they would like to add any additional Accounts
    1. If the User selects Yes to add another Account, the form clears and are asked to add another Account
    2. If the User selects No, they are taken to `/onboarding/2`
  2. When a User attempts to submit the form and their is an error, they stay on the form with the last information they attempted to submit and an error message is rendered at the top (if a particular form field is responsible for the error, it is highlighted as well)

1. When a User navigates to `/onboarding/2`, they are shown first shown a form asking them to enter the information for their first Hoop
  1. When a User submits the form successfully, the new Hoop is saved & they are shown a modal asking if they would like to add any additional Hoops
    1. If the User selects Yes to add another Hoop, the form clears and are asked to add another Hoop
    2. If the User selects No, they are taken to `/tour/dashboard`
  2. When a User attempts to submit the form and their is an error, they stay on the form with the last information they attempted to submit and an error message is rendered at the top (if a particular form field is responsible for the error, it is highlighted as well)

1. When a User navigates to `/tour/dashboard`, they are shown the same page as `/dashboard`, but overlaid with a series of instruction tips pointing to features with the option to see the next tip or exit the tour
  1. When a User selects exit, they are taken to `/dashboard`
  1. When a User selects next, they are shown the next tip until the reach the last tip
  2. When a User selects next on the final tip, they are taken to `/tour/transactions`

1. When a User navigates to `/tour/transactions`, they are shown the same page as `/dashboard`, but overlaid with a series of instruction tips pointing to features with the option to see the next tip or exit the tour
  1. When a User selects exit, they are taken to `/transactions`
  1. When a User selects next, they are shown the next tip until the reach the last tip
  2. When a User selects next on the final tip, they are taken to `/tour/hoops`

1. When a User navigates to `/tour/hoops`, they are shown the same page as `/dashboard`, but overlaid with a series of instruction tips pointing to features with the option to see the next tip or exit the tour
  1. When a User selects exit, they are taken to `/hoops`
  1. When a User selects next, they are shown the next tip until the reach the last tip
  2. When a User selects next on the final tip, they are taken to `/dashboard`

1. When a User first logs in, they are taken to `/dashboard`.

1. When a User navigates to any page, they are shown a Header containing the Hoops branding, their Safe-to-Spend Balance widget, an username indicating they're logged in, and a top-level navigation menu
  1. When a User clicks the Hoops branding, they are taken to `/dashboard`
  2. When a User clicks their username, they are taken to `/user-information`
  2. When a User clicks on the Balance section, the "widget" is expanded to show the calculation of their "Safe-to-Spend" balance
  2. When a User clicks on the header of the expanded Balance widget, it is collapsed back to just show the "Safe-to-Spend" balance
  2. When a User clicks on the "allocated in Hoops" section of the expanded Balance widget, they are taken to `/hoops`
  2. When a User clicks on the "available balance", "scheduled transactions", or "pending transactions" lines of a particular account in the expanded Balance widget, they are taken to `/accounts/:id/`

2. When a User navigates to `/dashboard`, they shown a list of their Hoops w/ current allocated amounts, positioned above a list of all recent Transactions, a list of all Accounts in a side bar w/ respective balances
  1. When a User clicks on the header for the Hoops section, they are taken to `/hoops`
  2. When a User clicks on the header for the Transaction section, they are taken to `/transactions`
  2. When a User clicks on the header for the Account section, they are taken to `/accounts`
  2. When a User clicks on the account name or "available balance" of particular account in the Account section, they are taken to `/accounts/:id/` for that particular account

2. When a User navigates to `/user-information`, they are shown their username, email address, an option to change their password, and an option to delete their account
  1. When a User clicks the edit icon next their username & email address, they are given the opportunity to edit them
  2. When a User clicks the option to change their password, they are asked to supply their current password and type the new one twice, then asked to log in again after hitting submit
  3. When a User clicks the option to delete their account, they are told their information will not be recoverable and asked if they are sure

1. When a User navigates to `/accounts/`, they are shown a list of all of their accounts with name, bank, & available balance for each one
  2. When a User clicks on the account name or "available balance" of particular account in the list, they are taken to `/accounts/:id/` for that particular account
  2. When a User clicks the add new account button at the bottom of the list, they are taken to `/accounts/new`

1. When a User navigates to `/accounts/new`, they are shown a form for entering the new Account's information
  1. When a User submits the form successfully, the new Account is saved & they are taken to `/accounts` where they can see the new account in their list of accounts
  2. When a User attempts to submit the form and their is an error, they stay on the form with the last information they attempted to submit and an error message is rendered at the top (if a particular form field is responsible for the error, it is highlighted as well)

1. When a User navigates to `/accounts/:id`, they are shown the detailed information for that account, given the option to edit information for the account, and given the option to close the account
  6. When a User edits the name of an Account, the Account is updated & they are shown the updated Account
  7. When a User marks an Account as closed, the account is updated & they are taken back to `/accounts` where they can no longer see the closed account in the list

1. When a User navigates to `/transactions`, they are shown a paginated list of all Transactions for all accounts sorted by date/time descending
  2. When a User views a list of Transactions, they are shown the timestamp, payee, account, spent-from Hoop, & amount for each Transaction
  2. When a User clicks the "spent-from" detail on a Transaction, they are given a quick option to edit the spent-from Hoop
  17. When a User marks a Transaction as "spent from" a given Hoop, the Transaction's information is updated in the list, _**but they can only select a Hoop if there are enough funds available in the Hoop**_
  3. When a User views a list of Transaction, they are shown buttons for searching and filtering the displayed Transactions
  3. When a User navigates away from a searched/filtered list of Transactions, the state is preserved and they are shown the same results when the navigate back to the list
  1. When a User views a list of Transactions, they are shown a form for adding a new Transaction as the first item in the list
  4. When a User correctly (e.g. no client-side validation errors such as missing required field or invalid data type) fills out the information in the new Transaction form at the top of the list, it is immediately added to the list of Transaction below, but shown to be pending submission
  5. When a User's newly submitted Transaction is accepted by the server, the pending submission indicator goes away and the Transaction appears normally in the list
  6. When a User's newly submitted Transaction fails to submit, the pending submission indicator is replaced with an error indicator sharing a brief error message and an option to edit, delete, or retry
  7. When a User clicks the option to edit a failed new Transaction submission, it turns back into a form while still showing the error message & allows the user to update the data before re-submitting it (or canceling and deleting the new Transaction)
  8. When a User clicks the option to retry a failed new Transaction submission, it goes back to pending state until it resolves
  9. When a User clicks the option to delete a failed new Transaction submission, it is removed from the list
  10. When a User clicks a resolved Transaction in the list, the Transaction is expanded to show details & the URL is navigated to `/transactions/:id`

1. When a User navigates to `/transactions/:id`, they are scrolled to its position in the unfiltered list of all Transaction and the Transactions details are shown, including description, geolocation, & optional notes & attachments and they are given an option to edit the Transaction's information
  1. When a User clicks the option to edit a Transaction's information, they are taken to `/transactions/:id/edit`

1. When a User navigates to `/transactions/:id/edit`, they are shown a form where they can edit its information or delete the Transaction
  1. When a User submits the form successfully, they are taken back to `/transactions/:id`
  1. When a User submits the form and it fails, they are shown an error message and any relevant fields in the form are highlighted
  10. When a User deletes a Transaction, the Transaction is deleted and they are taken back `/transactions` with all previous filters intact

7. When a User navigates to `/hoops`, they are shown a list of all Hoops and an option to add a new Hoop
  1. When a User clicks the option to create a new Hoop, they are taken to `/hoops/new`
  2. When a User drags from the Safe-to-Spend Balance widget to a Hoop in the list, they are asked how much money to move to that Hoop
  1. When a User moves money from their Safe-to-Spend Balance to a Hoop, the Safe-to-Spend widget updates & the Hoop updates
  2. When a User drags from a Hoop in the list to the Safe-to-Spend Balance widget, they are asked how much money to take from that Hoop
  13. When a User moves funds from a Hoop back to their Safe-to-Spend Balance, the Safe-to-Spend widget updates & the Hoop updates
  2. When a User drags from a Hoop in the list to another Hoop in the list, they are asked how much money to move from one to the other
  3. When a User moves funds from into one Hoop from another, both Hoops are updated
  10. When a User clicks a Hoop in the list, the Hoop is expanded in the list and the URL navigates to `/hoops/:id`

1. When a User navigates to `/hoops/new`, they are shown a form for entering the new Hoop's information
  1. When a User submits the form successfully, the new Hoop is saved & they are taken to `/hoops` where they can see the new account in their list of accounts
  2. When a User attempts to submit the form and their is an error, they stay on the form with the last information they attempted to submit and an error message is rendered at the top (if a particular form field is responsible for the error, it is highlighted as well)

8. When a User navigates to `/hoops/:id`, they are scrolled to its position in the list (if not already there) of all Hoops and the Hoop details are shown, including the name, a list of Transactions Spent From the Hoop, a graph/list of times money was moved in/out of the Hoop (and where from/to) and they are given an option to edit the Hoop's information
  1. When a User clicks edit on a Hoop, the Hoop's name is changed to a form to edit it, the URL is navigated to `/hoops/:id/edit`, and they are shown an option to archive the Hoop
  2. When a User submits the new Hoop name, it changes back from a form to a header and the URL is navigated back to `/hoops/:id`
  2. When a User clicks the option to archive a Hoop, they are told the action will make it so they can no longer mark Transaction as Spent From the Hoop & all remaining funds in the Hoop will be moved to the Safe-to-Spend balance and asked if they are sure
  4. When a User confirms Hoop archival, they are taken back to `/hoops` and the archived Hoop no longer shows up in the list
  5. When a User rejects Hoop archival, they confirmation question goes away and they remain on `/hoops/:id/edit`

### Stretch Goals:

> NOTE: The following from the previous API-only project still need updated, but they begin to get at the idea of what else I still want to do with the project.

Eventually, I'd like the application to build the following stories as well:

1. When a User creates a Shared User, they are given the new Shared User
  1. When a User invites another User to join a Shared User, they are given a success message
  28. When a User accepts an invitation to join a Shared User, they are given the Shared User
  28. When a User changes profiles to manage the Accounts, Transactions, & Hoops (i.e. do stories & sub-stories 1 through 5) of their Shared User (if they have one)
  29. When a User requests to leave a Shared User, they are given the old Shared User id & name
  30. When a User votes to delete the data of a Shared User, they are given a success message (and the other member Users are sent a notification; if all agree, then it will be deleted)

1. When a User imports Transactions from a csv, the Transactions are added to a given Account & they are given the updated list of Transactions
2. When a User signs up for new Transactions to be auto-imported from participating bank accounts (using Plaid), Transactions are added to the account as they are received from Plaid & the User is given a success message
3. When a User creates a subset of a Hoop, called a Goal, they are given the new Goal
  1. When a User sets a target date for a Goal (but a Goal doesn't have to have a target date), they are given the updated Goal
  2. When a User requests to automatically schedule money to be moved into a Goal, they are given the updated Goal
  3. When a User sets the priority on an Goal, they are given the updated Goal
  4. When a User moves money in and out of being reserved for a Goal, they are given the updated Goal & Balance of the Hoop or Balance the money is moved in or out of

4. When a User creates a subset of a Hoop, called an Expense, they are given the new Expense
  1. When a User sets a frequency for the Expense to reoccur, they are given the updated Expense
  2. When a User requests to automatically schedule money to be moved into an Expense, they are given the updated Expense
  3. When a User sets a priority on an Expense, they are given the updated Expense
  4. When a User moves money in and out of being reserved for the next occur date for an Expense, they are given the updated Expense & Balance of the Hoop or Balance the money is moved in or out of
  5. When a User moves money in and out of being reserved for the currently available funds on an Expense, they are given the updated Expense & Balance of the Hoop or Balance the money is moved in or out of
