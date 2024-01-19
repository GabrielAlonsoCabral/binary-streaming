import {  log } from "console";
import { BinaryStreaming } from "./index";

log("[BinaryStreaming] - Test started!")

const binaryStreaming = new BinaryStreaming();

const textMessage ="Gabriel Cabral"
const textMessageInBinary = "1000111 1100001 1100010 1110010 1101001 1100101 1101100 100000  1000011 1100001 1100010 1110010 1100001 1101100 "

const messageInBinary = binaryStreaming.textToBinary(textMessage);
log("Test: [textToBinary] - status: ", messageInBinary===textMessageInBinary ? "Ok" :"Failed")
const messageInText = binaryStreaming.binaryToText(messageInBinary);
log("Test: [binaryToText] - status: ", messageInText===textMessage ? "Ok" :"Failed")

log("Test: [decimalToBinary] - status(1): ", binaryStreaming.decimalToBinary(10)==="1010" ? "Ok" :"Failed")
log("Test: [decimalToBinary] - status(2): ", binaryStreaming.decimalToBinary(9)==="1001" ? "Ok" :"Failed")
log("Test: [decimalToBinary] - status(3): ", binaryStreaming.decimalToBinary(26)==="11010" ? "Ok" :"Failed")

log("[BinaryStreaming] - Test finished!")
