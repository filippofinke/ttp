# Traveling Thief Problem - Gecco 2024

## Introduction

The Traveling Thief Problem is defined as follows. Given is a set of cities $ N = \{1, . . . , n\} $ for which a distance $ d*{ij} $, $ i, j \in N $ between any pair of cities is known. Every city $ i $ but the first contains a set of items $ M_i = \{1, . . . , m_i\} $. Each item $ k $ positioned in the city $ i $ is characterized by its value $ p*{ik} $ and weight $ w*{ik} $, $ I_i \sim (p*{ik}, w*{ik}) $. The thief must visit each of the cities exactly once starting from the first city and returning back to it in the end. Any item may be picked up into the knapsack in any city until the total weight of collected items does not exceed the maximum possible weight $ W $. A renting rate $ R $ is to be paid per each time unit being on a way. $ \nu*{\text{max}} $ and $ \nu\_{\text{min}} $ denote the maximal and minimum speeds that the thief can move, respectively. The goal is to find a tour of the maximal profit.

## Authors

- [Filippo Finke](https://github.com/filippofinke)
- [Walter Sostene Losa](https://github.com/enetsos)
