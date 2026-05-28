# ADR-0003: Protocol Folder Owned by Bot

**Date:** 2026-05-27  
**Status:** proposed  
**Level:** L1

## Problem

A separate protocol repository added coordination overhead before the protocol had independent release needs. The bot is the local authority boundary all public edge surfaces enter.

## Decision

Recombine protocol contracts into `bicameral-bot/protocol/`. The protocol folder owns schemas, conformance fixtures, object vocabulary, and compatibility notes used by bot, MCP, integrations, and cloud clients.

## Non-Goals

This does not make integrations or MCP subpackages of the bot. They remain separate repos that depend on bot-owned protocol contracts.

## Consequences

The standalone `bicameral-protocol` repo becomes a deprecated pointer.
