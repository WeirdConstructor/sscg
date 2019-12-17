# SSCG - Game Design Document

## Good Tree
- Good Tree
  - Resources
    - Raw Rocks
      => can be analyzed
    - Minerals / Rare Eart Elements
      => can only be found either by analyzing or mined with a special device,
      they are also distributed randomly
    - Petrols
      => Can only be bought at planet stations (producers).
      Can be sold anywhere with a highly fluctuating price (daily swings up/down)
      - Crude Oil
      - Gasoline
      - Kerosine
    - Building Materials
      - Bricks
        => Buy: Planet, Sell: Planet
      - Metals
        => Buy: Planet, Sell: Station/Planet
  - Food
    - Basic Foods
    - Specialty Foods
    - Drinks
      - Drinking water
        => Buy: Planet, Sell: Station, and desert Planet for more good price.
      - Soft drinks
      - Alcoholic beverages
  - Entertainment
    => Buy: Anywhere, Sell: Anywhere
    But you can only sell for a good price depending on simulated trends.
    That means a high peak, and a slow decline with a long tail.
    Media merchants in principle buy everything. But they give you tips on what
    kind of media is currently in trend.
    - Books
    - Games
    - Movies / Series
  - Machines
    - Computers
    - Industry
  - Lifestyle
    - Furniture
    - Interior Decoration

## Good Category: Resources

- Resources are only sold and bought on stations at a planet.
- Make the generation of Systems/Planets spawn different distribution of resources.
  - The need factor of the planet influences whether the baseprices - it is either lowered or raised by this factor.
 
## Economy Fluctuations

- Make the Economy fluctuate between the systems, close systems should have only small differences, further away more variance
  - This means the systems need to be distributed across a 2D map and an interpolated noise overlays it for the base price influence.
- The fluctionation influences the base prices of the goods.
- Also add **Per Good-Group fluctuations**, that only influence some specific categorys/groups

## Reputation

- Based on Transaction-Points.
- 10 Levels of Reputation (RP-Level)
   - Lvl1: 1
   - Lvl2: 10
   - Lvl3: 100
   - Lvl4: 500
   - Lvl5: 1000
   - Lvl6: 2500
   - Lvl7: 5000
   - Lvl8: 10000
   - Lvl9: 50000
   - Lvl10:100000 TPs

### Merchant Reputation

- You get a few TPs for regular buy/sell
- You get a specified amount of TPs for contracts
- **Balancing: Contracts give more TPs but not that much money, as you are only transporting for the merchant!**
- *Question:* How does the RP-Level influence the available goods and contracts?

#### System Trade Level

- With each TP you earn in that System your System Trade Level gets up.
- The ST-Level needs 10x the TP than the RP-Levels need
- The System Trade Level controls what other goods you can trade in that system, which is not yet fleshed out.

### Merchant Prices

- If a merchant sells many different goods, he tends to be less generous with his prices.
- If a merchant is more specialized, you get better prices.
  - Specialized merchants should be less common, or else the player only visits those


### Space Taxi

Depending on the wealth of the system the people you transport pay you more. There should be some
kind of negotiation mechanic, so that the price for people transport is not just like any other good
bound to a base price.

Depending on the luxury class of your ship, your overall reputation and wealth of the source
planet or system you can charge more. Price also depends on the deadline for the transport,
you can charge more for very soon delivery.

If you don't make it in time, you are paid 25% on the original prices less for each day you missed.

If you make it in time, there is a chance the customer gives you:

- A contract for some good delivery from the destination.
- A voucher for either buying or selling goods from/to a specific merchant.
- Introduces you to a new passenger you can negotiate with.
- even smaller chance: Introduces you to a new passenger for a fixed given price.

#### Negotiation

Customer tells you where they want to go, and by what time they want to be there.
You can enter an amount and the customer either says yes immediately or says no.
If they say no, you have 2 tries to make an alternative offer.

#### Customer generation

Ideally there is a function that just returns a list of waiting customers for each point in time.
As state we save the negotiation state with that customer or whether they have been picked up.


## World

The initial galaxy has only a few dozens of generated systems. Like 20-40 of them, depends
on how much content I can generate. The player can earn money in this set of generated systems
to beat the goal of the game. 

- Each system has about 1-3 stations, with at least 1 planet. 
- Some systems also have asteroids (the starting one always has them) with system dependend mixture
of resources in them.

### Manual Structure mining

Ancient alien structures are found in the systems. Aside from asteroid mining which can be done
automatically by the ship. For these you have to buy a drone for mining, which you steer through
the structure. Aside from raw resources you can sometimes also find processed resources or planetary
resources like oil and water.

Experience is like this:

- You arrive with the ship, and get the option to "launch drone"
- You get a drone-HUD, new control help and the option to get back to the ship (automatically)
- The drone allows you to also fly backwards and strafe left/right
- The structure is lit up by your drone (test a spotlight!)
- The block right in front of the drone is highlighted and you get displayed the resource
- On "transfer" the block is transferred into your inventory. Size of the blocks is 50cm, and volume 1/8th of 1m³.

## Goal

### Main Goal - Ship with Light Convolution Drive

The main goal is to afford a ship with a light convolution device that allows them
to travel to other galaxies.

- The prices for this ship is yet to be decided.
- The ship is the overall goal of the game, it should have the best specs in every way and is allowed to even break balancing. It's like the rocket goal in Factorio, after that you have beat the game.

### Paths to the Goal

There should be multiple paths, ideally for each good category. Each path starts with a merchant somewhere (pointers to them are directly in your "address book"). If you did a contract for him, you get notified a new contract. You collect "path tokens", when complete you get the ship.

You can stray off the path any time and still go for it later. 

The path should be designed to lead through the whole galaxy, so that you visit each station or at least system once.
