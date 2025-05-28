# easydeploy

**A simple tool for easily supporting zero-downtime deployments for docker compose projects.**

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

easydeploy is a tool designed to simplify and enable zero-downtime deployments for services managed with Docker Compose. It automates the process of upgrading your services without interrupting user access.

## The Problem It Solves

Deploying updates to your applications is often troublesome, and involves stopping your services, making them temporarily unavailable to your users. This downtime is frustrating to both you and your users. The goal of easydeploy is to autoamte and streamline this process, ensring that users can continue to use your services even while they are being updated.

## How easydeploy Helps

easydeploy streamlines this process by scaling up your services when a new update is available.This allows easydeploy to spin up replacement containers for your services before shutting down and removing the old ones. This ensures that your application remains available throughout the upgrade process.

## Features

* **Zero-Downtime Deployments:** The core feature! Update your services without interrupting users.
* **Simple Configuration:** Easy to integrate with your existing `docker-compose.yml` files.

## Prerequisites

* Docker (version >=26.1.5)

## Getting Started

### How to use

**Add to your docker-compose.yml**

```yaml
services:
  foo:
    image: someimage:latest
    labels:
      com.easydeploy.enabled: true #Adding this to any service enables zero-downtime deployments.
    deploy:
      replicas: 2
  easydeploy:
    image: ghcr.io/stroogle/easydeploy:latest
    volumes:
      - ./docker-compose.yml:/app/docker-compose.yml
      - /var/run/docker.sock:/var/run/docker.sock
      - ./deployment.logs:/tmp/logs #Optional
    environment:
      - TIMING: "*/2 * * * *" #Optional
```
