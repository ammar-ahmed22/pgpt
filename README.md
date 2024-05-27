<div align="center">
  <h1>Pretty GPT</h1>
  <p>Ask ChatGPT anything directly from the terminal with pretty markdown rendering! With support for previous chat messages!</p>
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

### Configuration (`config set`)
#### `model`
Set the default model to use when asking questions.
```bash
pgpt config set model <MODEL>
```
Currently supported options are `gpt-3`, `gpt-4`, and `gpt-4o`

#### `cache-length`
Sets the number (`positive integer`) of prompt/response pairs to save in cache. Think of this as your chat history. 
Defaults to 5.
```bash
pgpt config set cache-length <LENGTH>
```

#### `context`
Sets the number (`positive integer`) of previous prompt/response pairs to send with a query. If value is greater than `cache-length`, all saved values will be sent.
Defaults to 0.
```bash
pgpt config set context <NUMBER>
```

#### `api-key`
Sets the OpenAI API key.
```bash
pgpt config set api-key <API_KEY>
```

To display the configuration values for any of the above options use:
```bash
pgpt config show <OPTION>
```
or display all of the configuration values with `pgpt config show all`

#### Cache Clearing
As the cache (saved chat history) can become very long or irrelevant, you can clear the saved history with:
```bash
pgpt config clear cache
```

### Question
Asking a question to ChatGPT using saved configuration parameters
```bash
pgpt query what is the meaning of life
```

This will query ChatGPT using the `model` set in configuration and any chat history saved up to the `context` value.

#### `--cost`
Passing the `--cost` flag will display the total cost for the query using the model dependent prices.
```bash
pgpt query --cost how to do a for loop in Rust
```

#### `-m, --model`
Override the model set in configuration and use a specific model for the query
```bash
pgpt query -m gpt-4o how to reverse a linked list in C++
```

#### `-c, --context`
Override the context value set in the configuration.

For example, if the `context` in configuration is set to `3` but you only want to send the last message with the query. 
```bash
pgpt -c 1 Explain what you just said before this
```

#### `-s, --show-context`
Display the previous messages that are being sent with the query
```bash
pgpt -s elaborate further
```

## Examples
We'll use the default values set by CLI to start:

### Showing the configuration
```bash
pgpt config show all
```
```bash
Creating configuration file with default values at "/Users/ammar/Library/Application Support/com.pgpt.pgpt/./config.json"
Model: gpt-3
API Key (encrypted): ����p"��Ý�<H*���g�4ق
                                         �"O��?��
                                                 ��/���s��G�C�EFQ~����Y�t�K���=���x���T�d|���G�;Hud�
Cache Length: 5
Context: 0
To display cache, run `pgpt config show cache`
```

### Set the context to the max value
```bash
pgpt config set context 5
```
```bash
Setting context to 5
Saved config successfully!
```

### Ask a question
```bash
pgpt query what is a for loop
```
```bash
Saved cache successfully!
Cache capacity 1/5

Response from gpt-3.5-turbo-0125
A for loop is a control flow statement that allows you to iterate over a sequence of elements (such as a list, tuple, 
dictionary, etc.) and execute a block of code for each element in the sequence. It consists of three parts: 
initialization, condition, and increment/decrement. The loop continues to execute as long as the condition is true, 
and the increment/decrement part is used to update the loop variable.
```

### Show the current cache
```bash
pgpt config show cache
```
```bash
Cache:
Cached 1/1
You said: what is a for loop
GPT said:
A for loop is a control flow statement that allows you to iterate over a sequence of elements (such as a list, tuple, dictionary, etc.) and execute a block of code for each element in the sequence. It consists of three parts: initialization, condition, and increment/decrement. The loop continues to execute as long as the condition is true, and the increment/decrement part is used to update the loop variable.
```

### Ask a follow-up question (with the total cost and previous messages)
```bash
pgpt query --cost --show-context how do I do that in Rust
```

```bash
Saved cache successfully!
Cache capacity 2/5
You said:
what is a for loop
GPT said:
A for loop is a control flow statement that allows you to iterate over a sequence of elements (such as a list, tuple, 
dictionary, etc.) and execute a block of code for each element in the sequence. It consists of three parts: 
initialization, condition, and increment/decrement. The loop continues to execute as long as the condition is true, 
and the increment/decrement part is used to update the loop variable.

You said:
how do I do that in Rust

Response from gpt-3.5-turbo-0125
In Rust, you can use the for loop to iterate over a collection of items. Here is an example of a basic for loop in 
Rust:

fn main() {                       
    let numbers = [1, 2, 3, 4, 5];
                                  
    for number in numbers.iter() {
        println!("{}", number);   
    }                             
}                                 

In this example, the for loop iterates over each element in the numbers array and prints it out. The iter() method is 
used to create an iterator over the array.

You can also use ranges in for loops in Rust. Here's an example:

fn main() {               
    for i in 0..5 {       
        println!("{}", i);
    }                     
}                         

In this example, the for loop iterates over a range from 0 to 5 (excluding 5) and prints each number.

Cost: $0.000345
```

> [!NOTE]
> The above outputs are shown without the formatting that would be displayed in the terminal. Most terminals support ANSI colors which would display the responses from ChatGPT in terminal markdown.


<!-- ### Question
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
``` -->

## License
[MIT](./LICENSE)




