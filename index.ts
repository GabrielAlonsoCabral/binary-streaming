const {text_to_binary, binary_to_text}  = require("./index.node")

export class BinaryStreaming{
    textToBinary(text:string){
        return text_to_binary(text)
    }

    binaryToText(binaryText:string){
        return binary_to_text(binaryText)
    } 
}