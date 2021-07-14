https://stackoverflow.com/questions/61013311/how-do-i-handle-errors-in-warp-using-both-rejection-and-the-question-mark-operat

# Decentralized Gift Card Service
The goal of this service is to integrate with various e-commerce platforms in order to allow online stores to issue gift cards and store credit that is backed by blockchain technology.

## ECommerce Integration
For a minimum viable product, our aim is to integrate only with Shopify. In order to do this, we will need a Shopify Partners account which will allow us to generate a custom application as well as the API keys that we need.

When a shop keeper is looking to use our service, we will need to follow the authentication flow outlined in the [Shopify documentation](https://shopify.dev/apps/auth/oauth). We will need to decide what sort of scopes/permissions we will need to request of the shop.

After we are authenticated, our service will receive an access token from Shopify that needs to be saved. It will be valid as long as the shop has our service installed. When it comes time to do any sort of operations with the shop's API, we will need to request a [session token](https://shopify.dev/apps/auth/session-tokens). This flow will repeat whenever the token becomes invalid.

Throughout these two main flows, we will need to handle various scenarios in which something may go wrong. Here is a working list of things that could go wrong:
- The shop uninstalls the application (we would not be able to get a working session token)
- The shop does not complete the authentication process (database entries will be left incomplete)

## User Facing Flow
1. Shop keeper signs up for the Service
1. Service will create gift card products through a UI page/form
1. User will purchase a gift card, or shop keep can credit a user
1. Once order is considered complete, user will receive an email with special code
1. Shop keep can resend email if gift card is valid
1. Upon checkout, special code will be entered in gift card location
1. Charge is discounted, gift card value is debited
1. Upon usage of credit through payment, our job is done

## Questions
##### How does our system get notified that a gift card is purchased?
We do not want to have to poll the Shopify API repetitively to find out a card has been bought. Maybe we will be able to have a hook attached to our gift card products and the hook will notify our service.

##### Is giving the user a code the best possible way to handle customer authentication?
How do we validate a credited user when they are attempting to redeem their store credit? One way is to provide them with a code, preferably though email, that will be entered into the Shopify checkout page. Maybe there is a more modern way that utilizes a cookie or something? We will have to keep sharing gift cards in mind.

##### How do we get some sort of actionable element/button in the Shopify checkout page?
We will need to be able to make sure the user will be able to recognize their ability to use a gift card when checking out. Hopefully this will take them to a checkout page that comes out of the box with Shopify.

##### What entities need to exist within Shopify to utilize the Discount/Gift Card code feature?
It appears that within the checkout page, you have a text field to enter a code in order to redeem a discount or code. Is there some sort of hook that we can use to accept the value coming through from that field? We would need to catch and process the input and somehow return a *discount* object that would be understood by Shopify to lower the price.

When a purchase that utilizes a gift card occurs, we will need to identify this and respond accordingly (by debiting the account, that is). Again, we want to avoid polling the API to find an event; a hook would better serve our purpose.

# Blockchain Technology
We will use a popular blockchain, called Ethereum, to store our user's store credit in a persistent, decentralized manner. To do this, we will need to write a smart contract that will handle the logic of keeping a balance, as well as crediting and debiting appropriately.

Our plan is to utilize [Truffle](https://www.trufflesuite.com/truffle) to manage our contracts throughout testing, compilation, and deployment.

Here are the following functions that we require (note they are already in Solidity):
```
function balanceOf(string memory storeId, string memory clientId) public view returns (uint256)
function credit(string memory storeId, string memory clientId, uint256 credits) public returns (bool)
function redeem(string memory storeId, string memory clientId, uint256 credits) public returns (bool)
```
