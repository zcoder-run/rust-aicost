# MODEL PRICES

## Large Language Models (LLMs)

*Approximate number of tokens per $
| AI Model                                           | Current Speed(Tokens per Second) | Input Token Price(Per Million Tokens) | Output Token Price(Per Million Tokens) |
| -------------------------------------------------- | -------------------------------- | ------------------------------------- | -------------------------------------- |
| GPT OSS 20B 128k                                   | 1,000                            | $0.10(10M / $1)*                      | $0.50(2M / $1)*                        |
| GPT OSS 120B 128k                                  | 500                              | $0.15(6.67M / $1)*                    | $0.75(1.33M / $1)*                     |
| Kimi K2 1T 128k                                    | 200                              | $1.00(1M / $1)*                       | $3.00(333,333 / $1)*                   |
| Llama 4 Scout (17Bx16E) 128k                       | 594                              | $0.11(9.09M / $1)*                    | $0.34(2.94M / $1)*                     |
| Llama 4 Maverick (17Bx128E) 128k                   | 562                              | $0.20(5M / $1)*                       | $0.60(1.6M / $1)*                      |
| Llama Guard 4 12B 128k                             | 325                              | $0.20(5M / $1)*                       | $0.20(5M / $1)*                        |
| DeepSeek R1 Distill Llama 70B                 128k | 400                              | $0.75(1.33M / $1)*                    | $0.99(1.01M / $1)*                     |
| Qwen3 32B 131k                                     | 662                              | $0.29(3.44M / $1)*                    | $0.59(1.69M / $1)*                     |
| Mistral Saba 24B 32k                               | 330                              | $0.79(1.27M / $1)*                    | $0.79(1.27M / $1)*                     |
| Llama 3.3 70B Versatile 128k                       | 394                              | $0.59(1.69M / $1)*                    | $0.79(1.27M / $1)*                     |
| Llama 3.1 8B Instant 128k                          | 840                              | $0.05(20M / $1)*                      | $0.08(12.5M / $1)*                     |
| Llama 3 70B 8k                                     | 330                              | $0.59(1.69M / $1)*                    | $0.79(1.27M / $1)*                     |
| Llama 3 8B 8k                                      | 1,345                            | $0.05(20M / $1)*                      | $0.08(12.5M / $1)*                     |
| Gemma 2 9B 8k                                      | 500                              | $0.20(5M / $1)*                       | $0.20(5M / $1)*                        |
| Llama Guard 3 8B 8k                                | 765                              | $0.20(5M / $1)*                       | $0.20(5M / $1)*                        |

# MODEL NAMES

# Supported Models

Explore all available models on GroqCloud.

## [Featured Models](#featured-models)

[### OpenAI GPT-OSS 20B

GPT-OSS 20B is OpenAI's compact open-weight language model with 20 billion parameters, built in browser search and code execution, and reasoning capabilities.

Token Speed

\~1000 tps

Modalities

Capabilities](/docs/model/openai/gpt-oss-20b)

[### OpenAI GPT-OSS 120B

GPT-OSS 120B is OpenAI's flagship open-weight language model with 120 billion parameters, built in browser search and code execution, and reasoning capabilities.

Token Speed

\~500 tps

Modalities

Capabilities](/docs/model/openai/gpt-oss-120b)

## [Production Models](#production-models)

**Note:** Production models are intended for use in your production environments. They meet or exceed our high standards for speed, quality, and reliability. Read more [here](/docs/deprecations).

| MODEL ID                     | DEVELOPER | CONTEXT WINDOW                 (TOKENS) | MAX                 COMPLETION TOKENS | MAX FILE                 SIZE | DETAILS |
| ---------------------------- | --------- | --------------------------------------- | ------------------------------------- | ----------------------------- | ------- |
| gemma2-9b-it                 | Google    | 8,192                                   | 8,192                                 | -                             | Details |
| llama-3.1-8b-instant         | Meta      | 131,072                                 | 131,072                               | -                             | Details |
| llama-3.3-70b-versatile      | Meta      | 131,072                                 | 32,768                                | -                             | Details |
| meta-llama/llama-guard-4-12b | Meta      | 131,072                                 | 1,024                                 | 20 MB                         | Details |
| whisper-large-v3             | OpenAI    | -                                       | -                                     | 100 MB                        | Details |
| whisper-large-v3-turbo       | OpenAI    | -                                       | -                                     | 100 MB                        | Details |

## [Preview Models](#preview-models)

**Note:** Preview models are intended for evaluation purposes only and should not be used in production environments as they may be discontinued at short notice. Read more about deprecations [here](/docs/deprecations).

| MODEL ID                                      | DEVELOPER       | CONTEXT WINDOW                 (TOKENS) | MAX                 COMPLETION TOKENS | MAX FILE                 SIZE | DETAILS |
| --------------------------------------------- | --------------- | --------------------------------------- | ------------------------------------- | ----------------------------- | ------- |
| deepseek-r1-distill-llama-70b                 | DeepSeek / Meta | 131,072                                 | 131,072                               | -                             | Details |
| meta-llama/llama-4-maverick-17b-128e-instruct | Meta            | 131,072                                 | 8,192                                 | 20 MB                         | Details |
| meta-llama/llama-4-scout-17b-16e-instruct     | Meta            | 131,072                                 | 8,192                                 | 20 MB                         | Details |
| meta-llama/llama-prompt-guard-2-22m           | Meta            | 512                                     | 512                                   | -                             | Details |
| meta-llama/llama-prompt-guard-2-86m           | Meta            | 512                                     | 512                                   | -                             | Details |
| moonshotai/kimi-k2-instruct                   | Moonshot AI     | 131,072                                 | 16,384                                | -                             | Details |
| openai/gpt-oss-120b                           | OpenAI          | 131,072                                 | 32,766                                | -                             | Details |
| openai/gpt-oss-20b                            | OpenAI          | 131,072                                 | 32,768                                | -                             | Details |
| playai-tts                                    | PlayAI          | 8,192                                   | 8,192                                 | -                             | Details |
| playai-tts-arabic                             | PlayAI          | 8,192                                   | 8,192                                 | -                             | Details |
| qwen/qwen3-32b                                | Alibaba Cloud   | 131,072                                 | 131,072                               | -                             | Details |

## [Preview Systems](#preview-systems)

Systems are a collection of models and tools that work together to answer a user query.

  
  

**Note:** Preview systems are intended for evaluation purposes only and should not be used in production environments as they may be discontinued at short notice. Read more about deprecations [here](/docs/deprecations).

| MODEL ID           | DEVELOPER | CONTEXT WINDOW                 (TOKENS) | MAX                 COMPLETION TOKENS | MAX FILE                 SIZE | DETAILS |
| ------------------ | --------- | --------------------------------------- | ------------------------------------- | ----------------------------- | ------- |
| compound-beta      | Groq      | 131,072                                 | 8,192                                 | -                             | Details |
| compound-beta-mini | Groq      | 131,072                                 | 8,192                                 | -                             | Details |

  
  
[Learn More About Agentic Tooling

Discover how to build powerful applications with real-time web search and code execution](/docs/agentic-tooling)

  
  

Deprecated models are models that are no longer supported or will no longer be supported in the future. See our deprecation guidelines and deprecated models [here](/docs/deprecations).

  
  

Hosted models are directly accessible through the GroqCloud Models API endpoint using the model IDs mentioned above. You can use the `https://api.groq.com/openai/v1/models` endpoint to return a JSON list of all active models:

curlJavaScriptPython

Python

```
1import requests
2import os
34api_key = os.environ.get("GROQ_API_KEY")5url ="https://api.groq.com/openai/v1/models"67headers ={8"Authorization":f"Bearer {api_key}",9"Content-Type":"application/json"10}1112response = requests.get(url, headers=headers)1314print(response.json())
```

### Was this page helpful?

YesNoSuggest Edits