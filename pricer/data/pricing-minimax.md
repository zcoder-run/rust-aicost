| **Model**              | **Input** $ / M tokens | **Output** $ / M tokens | **Prompt caching Read** $ / M tokens |
| :--------------------- | :--------------------: | :---------------------: | :----------------------------------: |
| **MiniMax-M3.1-Flash** |         $0.15          |          $0.60          |                $0.015                |

**Standard**

| **Model**                          | **Input** $ / M tokens | **Output** $ / M tokens | **Prompt caching Read** $ / M tokens |
| :--------------------------------- | :--------------------: | :---------------------: | :----------------------------------: |
| **MiniMax-M3** ≤ 512k input tokens |         $0.30          |          $1.20          |                $0.06                 |
| **MiniMax-M3** > 512k input tokens |         $0.60          |          $2.40          |                $0.12                 |

**Priority**\*

| **Model**                          | **Input** $ / M tokens | **Output** $ / M tokens | **Prompt caching Read** $ / M tokens |
| :--------------------------------- | :--------------------: | :---------------------: | :----------------------------------: |
| **MiniMax-M3** ≤ 512k input tokens |         $0.45          |          $1.80          |                $0.09                 |
| **MiniMax-M3** > 512k input tokens |         $0.90          |          $3.60          |                $0.18                 |
