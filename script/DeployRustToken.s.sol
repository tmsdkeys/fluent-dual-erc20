// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

import {Script, console} from "forge-std/Script.sol";
import {IRustToken} from "../out/RustToken.wasm/interface.sol";

contract DeployRustToken is Script {
    function run() external {
        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY");
        vm.addr(deployerPrivateKey);
        
        // Configuration - you can modify these values
        bytes memory tokenName = "0x466c75656e745275737479"; // "FluentRusty"
        bytes memory tokenSymbol = "0x4652555354"; // "FRUST"
        uint256 initialSupply = 1000000000000000000000000; // 1,000,000 tokens with 18 decimals
    
        
        vm.startBroadcast(deployerPrivateKey);

        // Deploy WASM RustToken
        bytes memory wasmBytecode = vm.getCode("out/RustToken.wasm/foundry.json");
        console.log("WASM bytecode size:", wasmBytecode.length);
        
        address rustToken;
        assembly {
            rustToken := create(0, add(wasmBytecode, 0x20), mload(wasmBytecode))
        }
        
        require(rustToken != address(0), "RustToken deployment failed");
        console.log("RustToken deployed at:", rustToken);
        
        console.log("Initializing Rust Token contract...");
        console.log("Contract Address:", rustToken);
        
        // Call the initialize function on the deployed WASM contract
        IRustToken(rustToken).initialize(tokenName, tokenSymbol, initialSupply);
        
        console.log("Token initialization completed!");
        
        vm.stopBroadcast();
    }
}
