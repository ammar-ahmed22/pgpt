<div align="center">
  <h1>Pretty GPT</h1>
  <p>Ask ChatGPT anything directly from the terminal with pretty markdown rendering!</p>
  <p>Inspired by <a href="https://github.com/rigwild/shell-gpt-rs">shell-gpt-rs</a></p>
  <p>
    <a href="https://crates.io/crates/pgpt">
      <img src="https://img.shields.io/crates/v/pgpt" />
    </a>
    <a href="https://crates.io/crates/pgpt">
      <img src="https://img.shields.io/crates/d/pgpt" />
    </a>
  </p>
</div> 

## Demo
<img align="center" src="./demo.gif" width="1000" alt="GIF showcasing how to use pgpt CLI" >

## Installation
### Cargo
```bash
cargo install pgpt
pgpt --help
```

### Source
```bash
git clone https://github.com/ammar-ahmed22/pgpt.git
cd pgpt
cargo install --path .
```

## Usage
When using for the first time, pgpt will prompt you to enter an [OpenAI API key](https://platform.openai.com/api-keys).

The key is saved/encrypted in a local config file to be used later. 

Alternatively, you can pass your API key to the environment variable `OPENAI_API_KEY` in order to not save it.

### Question
Ask a question to ChatGPT (defaults to `gpt-3.5-turbo`)
```
pgpt <QUERY>...
```

### `-m, --model`
Select a specific model to use for ChatGPT.
```bash
pgpt -m, --model <MODEL> <QUERY>...
```

Available options for model:
| Option | ChatGPT Model |
| ------ | ------------- |
| `gpt-3` | `gpt-3.5-turbo` |
| `gpt-4` | `gpt-4-turbo` |
| `gpt-4o` | `gpt-4o` |

### `--cost`
Calculate the total cost associated with prompt and response:
```bash
pgpt --cost <QUERY>...
```

ChatGPT API pricing is dependent on the model with different costs for the different models.

Costs are based on "tokens" used for input/output. Each "token" roughly translates to 4 characters of text.

You can see pricing details [here](https://openai.com/api/pricing/)

### `--clear`
Clears the local config including the OpenAI API key
```bash
pgpt --clear
```

### `--help`
```bash
pgpt --help
```

```bash
Ask ChatGPT anything directly from the terminal with pretty markdown rendering!

Usage: pgpt [OPTIONS] [QUERY]...

Arguments:
  [QUERY]...  The query to ask ChatGPT

Options:
  -m, --model <MODEL>  ['gpt-3'] The model to use (optional) [options = 'gpt-3', 'gpt-4', 'gpt-4o']
      --clear          Clear local config including OpenAI API key
      --cost           Display the total cost for the prompt and response
  -h, --help           Print help
  -V, --version        Print version
```

## License
[MIT](./LICENSE)




