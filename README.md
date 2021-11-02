# Pro Filetype Parser
A custom file parser for the .pro format. It can parse the .pro file and execute the appropriate actions.

**DISCLAIMER**: The only goal for this project is to help me learn Rust.  That being the case, most of it is **not yet implemented!** That being said, any feedback on how the it is performing and how it could be improved is **greatly** apreciated.

## Pro filetype uses:
* Discord Messages Automation
* **[Coming soon]**

## Pro filetype format:
sample_command.pro
```json
{
  "type": "DM",
  "data": {
    "recipient": "FedoraMan",
    "amount": 10,
    "content": "Sup. Am pro",
    "vary": true
  }
}
```
### Property information:
The Pro format is structured as a JSON object with the following properties:
* **[Type]**: Specifies the action that will be performed.
	* **Values**: DM
* **[Data]**: Additional information required by the type.
	* **Values**: 
		* **DM**:
			* *Recipient*: Reciever of the messages;
			* *Amount*: How many messages to send;
			* *Content*: The message to send;
			* *Vary*: Vary the capitalisation of the *content*;

## How to use: _[Coming soon]_
1. Download pro_runner.exe.
2. Run the app inside of a terminal and pass the path **[ABSOLUTE]** to the .pro file.
3. The app will then run you through the necessary steps.
