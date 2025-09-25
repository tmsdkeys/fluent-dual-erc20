// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

import {Script, console} from "forge-std/Script.sol";
import {MyToken} from "../src/MyToken.sol";

contract DeployTokens is Script {
    function run() external {
        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY");
        address deployer = vm.addr(deployerPrivateKey);
    
        
        vm.startBroadcast(deployerPrivateKey);

        // Deploy WASM RustToken
        bytes memory wasmBytecode = vm.getCode("out/RustToken.wasm/foundry.json");
        console.log("WASM bytecode size:", wasmBytecode.length);
        
        // Example constructor args (adjust types/order to your constructor)
        string memory name = "TestRustToken";
        string memory symbol = "tRUST";
        uint256 decimals = 18;
        uint256 initialSupply = 1_000_000;

        // Append ABI-encoded constructor args to the creation code
        bytes memory creationByteCode = abi.encodePacked(
            wasmBytecode,
            abi.encode(name, symbol, decimals, initialSupply)
        );

        address rustToken;
        assembly {
            rustToken := create(0, add(creationByteCode, 0x20), mload(creationByteCode))
        }
        
        require(rustToken != address(0), "RustToken deployment failed");
        console.log("RustToken deployed at:", rustToken);
        
        console.log("Initializing Rust Token contract...");
        console.log("Contract Address (Rust Token):", rustToken);

        // Deploy Solidity ERC20 Token
        name = "TestSolToken";
        symbol = "tSOLT";
        initialSupply = 5_000_000; // Will be scaled by 10**decimals() in constructor

        MyToken solToken = new MyToken(name, symbol, initialSupply, deployer);
        console.log("SolToken deployed at:", address(solToken));
        console.log("SolToken owner:", solToken.owner());
        console.log("SolToken totalSupply:", solToken.totalSupply());
        
        vm.stopBroadcast();
    }
}