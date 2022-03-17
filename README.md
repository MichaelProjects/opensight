# Opensight

### Still under heavy construction
Opensight is still under development and there will be big & breaking changes in the future. This should only be a pre-alpha test release, with a [demo](https://demo.opensight.io/) to check it out.

## What is Opensight
Opensight is an Application Insight Platform, and should be a better alternative to other analytics and Application Insight Platform. You are able to host it On-Premis, or in the cloud managed by Opensight.

We support features like: 

- Performance Monitoring
- Error analysis
- Event-Tracking
- Device and User analytics

## How does it work
Opensight consists out of 2 parts, the Client and the Backend. The Backend is build in an microservice-architecture, the advantages of that is, if you dont want to use the whole Opensight-Stack you are free to choose which services you like, and dont need to pay for unused features.


| Services          | Version  | Ci  |
|-|-|-|
|[Core-Service](https://github.com/MichaelProjects/opensight/tree/master/core) | 0.3.1 | ![Pipeline](https://github.com/MichaelProjects/opensight/actions/workflows/core.yml/badge.svg)
|[Analytic-Service](https://github.com/MichaelProjects/opensight/tree/master/analytics_api) | 0.4.7      |![Pipeline](https://github.com/MichaelProjects/opensight/actions/workflows/analytics.yml/badge.svg)
|[Error-Service](https://github.com/MichaelProjects/opensight/tree/master/error_api)      | in construction      |![Pipeline](https://github.com/MichaelProjects/opensight/actions/workflows/error.yml/badge.svg) |
|[Event-Service](https://github.com/MichaelProjects/opensight/tree/master/event_api)      | in construction      |![Pipeline](https://github.com/MichaelProjects/opensight/actions/workflows/event.yml/badge.svg)  |
[Dashboard](https://github.com/MichaelProjects/opensight/tree/master/dashboard) | 0.1 | ![Pipeline](https://github.com/MichaelProjects/opensight/actions/workflows/dashboard.yml/badge.svg)

## SDK'S
Currently we offer following SDK's:
- Flutter SDk

Comming soon:
- IOS SDK (Swift)
- Android SDK (Kotlin)
- Javascript / TypeScript SDK

## How to deploy
Check out our [documentation](https://docs.opensight.io) to get started with your deployment of opensight.

## Want to contribute or contact me ?

Please contact me under info@vercise.io


