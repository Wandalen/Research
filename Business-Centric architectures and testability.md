# Table of contents

- [Software Design](#software-design)
  - [Characteristics of a Poor Software Design](#characteristics-of-a-poor-software-design)
  - [What makes a good Software Architecture?](#what-makes-a-good-software-architecture)
  - [Why is Software Architecture important?](#why-is-software-architecture-important)
  - [5 pieces of advice on Software Architecture](#5-pieces-of-advice-on-software-architecture)
- [Common Software Architecture Patterns](#common-software-architecture-patterns)
  - [What is an architectural pattern?](#what-is-an-architectural-pattern)
  - [What is the importance of software architecture pattern?](#what-is-the-importance-of-software-architecture-pattern)
  - [Layered Architecture Pattern](#layered-architecture-pattern)
  - [Event-driven Architecture Pattern](#event-driven-architecture-pattern)
  - [Microkernel Architecture Pattern](#microkernel-architecture-pattern)
  - [Microservices Architecture Pattern](#microservices-architecture-pattern)
  - [Space-Based Architecture Pattern](#space-based-architecture-pattern)
  - [Client-Server Architecture Pattern](#client-server-architecture-pattern)
  - [Master-Slave Architecture Pattern](#master-slave-architecture-pattern)
  - [Pipe-Filter Architecture Pattern](#pipe-filter-architecture-pattern)
  - [Broker Architecture Pattern](#broker-architecture-pattern)
  - [Peer-to-Peer Architecture Pattern](#peer-to-peer-architecture-pattern)
- [Domain Driven Design](#domain-driven-design)
  - [Isolating the domain: the layered architecture](#isolating-the-domain-the-layered-architecture)
  - [Expressing the model: Building Blocks](#expressing-the-model-building-blocks)
  - [Managing the life cycle of domain objects](#managing-the-life-cycle-of-domain-objects)
  - [Anti-corruption Layer pattern](#anti-corruption-layer-pattern)
- [Ports and Adapters Architecture](#ports-and-adapters-architecture)
  - [Ports](#ports)
  - [Adapters](#adapters)
  - [Application](#application)
  - [The Dependency Rule](#the-dependency-rule)
  - [Why use ports and adapters architecture?](#why-use-ports-and-adapters-architecture)
- [Clean Architecture](#clean-architecture)
  - [The Dependency Rule](#the-dependency-rule)
  - [Entities](#entities)
  - [Use Cases](#use-cases)
  - [Interface Adapters](#interface-adapters)
  - [Frameworks and Drivers.](#frameworks-and-drivers)
  - [Only Four Circles?](#only-four-circles)
  - [Crossing boundaries.](#crossing-boundaries)
  - [What data crosses the boundaries.](#what-data-crosses-the-boundaries)
- [Testing Strategy for Hexagonal Applications](#testing-strategy-for-hexagonal-applications)
  - [Unit testing](#unit-testing)
  - [Integration testing](#integration-testing)
  - [System testing](#system-testing)
  - [Acceptance testing](#acceptance-testing)
  - [Prefer unit tests](#prefer-unit-tests)

# Software Design
## Characteristics of a Poor Software Design
Common challenges that a poor software design have:

- Breaking of one functionality during development of another one.
- Existing code not supporting future enhancements.
- Complexity in understanding and maintaining the code.
- Difficult to test feature in isolation
- Difficult to add new features

The easiest approach to avoid mistakes is to understand and identify Bad programming habits and adapt good ones to make application development better.

![](img/Business-Centric%20architectures%20and%20testability/bad-design-characteristics.jpeg)

According to Robert C. Martin, recognized for developing Software Design Principles, There are 4 important characteristics of a bad design. Those are:

- **Rigidity**
- **Immobility**
- **Fragility**
- **Viscosity**

### **Rigidity**
If every small change in one class translates into cascading changes in other dependent classes then the coupling exists in the code. This makes the code Rigid.

Disadvantages of Rigidity in design:

1. would be difficult to give an approximation for a change
1. would lead to testing of more components than expected
1. would make managers apprehensive of allowing minor changes and in turns live with flaws in the application

### Immobility
` `When the components are tightly coupled or highly dependent on each other and can not be reused in another place, the code is said to be Immobile.

Disadvantages of Immobility in design:

1. Difficult to reuse most common functionalities.
1. If it is required to reuse the module, efforts and risk of separating it from original design will be high
1. Leads to creation of copies of the same code in multiple places in the projects This also means that bugs in original code are duplicated. Fixing this bug in original code does not fix it automatically in other places. Separate efforts are required to fix them

### Fragility
The design is Fragile, when any new change in code is breaking unexpected parts of the system. Such software is very difficult to maintain as one fix makes it worse by introducing more problems than solved.

Disadvantages of Fragility in design:

1. Some modules will be constantly on bug list
1. More time is consumed in fixing the bugs rather than fixing actual problems.
1. Programmers are reluctant to make changes in code as their confidence in such a piece of code is very low due to the problems they have encountered while fixing bugs.

### Viscosity
While implementing any change, there can be two ways of implementing fixes. One which preserves design and other a shortcut fix which breaks the design. Viscosity refers to the ease at which a developer can add design-preserving code to a system. If it is easier to add a hack than a design preserving code then the system has high viscosity.

Disadvantages of Viscosity in design:

1. The development environment is slow and inefficient
1. Leads to high compile time, long feedback time in testing and difficulty in integration

**A poor software design can be identified if the design has one more characteristic mentioned above.**

## What makes a good Software Architecture?
There are attributes of Software Architecture that one could call either good or not so good. The key elements are:

- The architecture meets the project requirements. A good Software Architecture fits the initial project requirements and can be adapted to any new ones. All good architectures plan for both today and tomorrow.
- The architecture is built for operational excellence. It's easily maintainable, flexible, and can be scaled up (or down) as needed. Good architectures also understand the relationship to organizational structure and know who will maintain them.
- The architecture is built for its users. It provides a user-friendly structure, whether it's for a developer or another end user, internally or externally. Performance has a significant impact on that user experience and should be considered here.

Architectures that meet current and future requirements, as well as build for both operations and user experience, are fair to call good Software Architectures.
## **Why is Software Architecture important?**
Now that we know what good Software Architecture looks like, we can dig more into why it is so important to the business.

**SA helps with early-stage decision-making**

The decisions you make at the very beginning of your project define every next step you take. Creating a solid Software Architecture first means having a well-defined project plan at your service.

You can also predict the qualities of your future system based on your SA. Moreover, with Software Architecture serving as your development guide down the road, it's easier to both fulfill your requirement and make any necessary changes.

**SA allows for making further changes and adjustments**

Speaking of changes, if the software has good architecture at its core, it will be much easier to implement new features and make adjustments to it.

It's inevitable that in the future, you'll want to add new functionality to your application based on tech advancement, customer demands, etc. A solid SA provides an understanding of what it takes to make a particular change and how to do it efficiently.

**SA serves as a communication link between stakeholders**

The Software Architecture allows all stakeholders, especially clients that are usually less tech-savvy, to better understand the project's structure and know what has to be done to complete it.

Considering that stakeholders usually have different concerns and priorities, SA acts as a common roadmap for them to agree on. Architecture is especially significant for complex software, which can be hard to understand for those not involved in IT.
## **5 pieces of advice on Software Architecture**
So how does an architect produce a good architectural design? Here are the five principles that have been successful in my experience:

1. Stick to your core requirements when creating Software Architecture. Avoid scope creep, map requirements to organizational structures, and design for scaling.
1. Don't fixate on a single pattern in the beginning. Only choose a specific one after you have an excellent overall understanding of your future software components.
1. Be ready to re-do the first version of your Software Architecture. Great software is designed to be deleted and iterated on. You won't get it right the first time.
1. Choose an estimation approach you feel more comfortable with. Every executive will want time estimates. Choose one that's realistic and be firm.
1. Focus on reusability. Avoid artisanal infrastructure by designing for repeat usage of everything from software libraries to container images. You are sure to reuse them.



# Common Software Architecture Patterns
## **What is an architectural pattern?**
An architectural pattern can be called an outline that allows you to express and define a structural schema for all kinds of software systems. It’s a reusable solution that provides a predefined set of subsystems, roles, and responsibilities, including the rules and roadmap for defining relationships among them. It helps you address various software engineering concerns such as performance limitations, high availability, minimizing business risk, etc.  

## **What is the importance of software architecture pattern?**
Software architecture patterns hold significant importance for it can solve various problems within different domains. For instance, instead of depending on a single server, complex user requests can be easily segmented into smaller chunks and distributed across multiple servers. In another example, testing protocols can be simplified by dividing various segments of the software rather than testing the whole thing at once.

Here are some more reasons why software architecture patterns are vital for any software application:

- Defining Basic Characteristics of an Application:**


Knowing each architecture’s characteristics, strengths, and weaknesses becomes important for choosing the right one to meet your business objectives. It has been observed that architecture patterns help in defining the basic characteristics and behaviors of an application. For instance, some architecture patterns can be naturally used for highly scalable applications, whereas others can be used for agile applications.

- Maintaining Quality and Efficiency:

There is a high possibility that any application you build might face quality issues. According to your software development quality attributes, selecting an architecture pattern can help minimize the quality issues while simultaneously maintaining efficiency.

- Providing Agility:**


It is natural for software applications to undergo numerous modifications and iterations during software development and even after production. Therefore, planning a core software architecture beforehand provides agility to the application and makes future moderations effortless.

- Problem Solving:**


Prior planning and knowledge of a software architecture give a clear idea of how the application and its components will function. With an architecture in place, the developing team can adopt the best practices to resolve complex processes and solve any errors in the future.

- Enhancing Productivity:**


Irrespective of the skills and knowledge one has about a programming language, framework, or application, there has to be certain standardized principles. With an appropriate application pattern in place, the company can quickly grasp the project’s status. In addition, productivity rates improve the moment an architecture pattern is in place to make the project scope clear.
## Layered Architecture Pattern

![](img/Business-Centric%20architectures%20and%20testability/layered-architecture-pattern.png)

You’ve probably heard of multi-layered, aka tiered architecture, or n-tier architecture. This architecture has gained popularity amongst designers and software architects alike for its commonalities with the conventional arrangements of IT communications in many startups and established enterprises. Often, a layered architecture is classified into four distinct layers: presentation, business, persistence, and database; however, the pattern is not confined to the specified layers and there can be an application layer or service layer or data access layer. Popular frameworks like Java EE utilized this architecture pattern.

Let’s say a software engineer is building a large application, and you found yourself employing all four layers to your architecture pattern. On the flip side, small businesses may combine the business and the persistence layers into a single unit, primarily when the latter is engaged as an integral part of the business logic layer components.

This pattern stands out because each layer plays a distinct role within the application and is marked as closed. It means that a request must pass through the layer right below it to go to the next layer. Another one of its concepts – layers of isolation – enables you to modify components within one layer without affecting the other layers.

To simplify this process, let’s take an example of an eCommerce web application. The business logic required to process a shopping cart activity, such as calculating the cart, is directly fetched from the application tier to the presentation tier. Here the application tier acts as an integration layer to establish seamless communication between the data and presentation layers. Additionally, the last tier is the data tier used to maintain data independently without the intervention of the application server and the business logic.

**Usage:**

- Applications that are needed to be built quickly.
- Enterprise applications that require traditional IT departments and processes.
- Appropriate for teams with inexperienced developers and limited knowledge of architecture patterns.
- Applications that require strict standards of maintainability and testability.

**Shortcomings:**

- Unorganized source codes and modules with no definite roles can become a problem for the application.
- Skipping previous layers to create tight coupling can lead to a logical mess full of complex interdependencies.
- Basic modifications can require a complete redeployment of the application.


## Event-driven Architecture Pattern

If you are looking for an architecture pattern that is agile and highly performant, then you should opt for an event-driven architecture pattern. It is made up of decoupled, single-purpose event processing components that asynchronously receive and process events. This pattern orchestrates the behavior around the production, detection, and consumption of all the events, along with the responses they evoke.

The event-driven architectural style consists of two topologies – mediator and broker. A mediator is used when multiple steps are needed to be orchestrated within an event bus through a central mediator. On the other hand, a broker is used to chain events together without using a central mediator.

![](img/Business-Centric%20architectures%20and%20testability/event-driven architecture.png)

A good example that uses event-driven architecture is an e-commerce site. The event-driven architecture enables the e-commerce website to react to various sources at a time of high demand. Simultaneously, it avoids any crash of the application or any over-provisioning of resources.

**Usage:**

- For applications where individual data blocks interact with only a few modules.
- Helps with user interfaces.

**Shortcomings:**

- Testing individual modules can only be done if they are independent, otherwise, they need to be tested in a fully functional system.
- When several modules are handling the same events, error handling becomes challenging to structure.
- Development of a system-wide data structure for events can become arduous if the events have different needs.
- Maintaining a transaction-based mechanism for consistency can become complex with decoupled and independent modules.
## Microkernel Architecture Pattern
This architecture pattern consists of two types of components – a core system and several plug-in modules. While the core system works on minimal functionality to keep the system operational, the plug-in modules are independent components with specialized processing.

![](img/Business-Centric%20architectures%20and%20testability/microkernel-architecture-pattern.png)

If we take a business application’s perspective, the core system can be defined as general business logic without the custom code for special cases, special rules, or complex conditional processes. On the other hand, the plug-in modules are meant to enhance the core system in order to produce additional business capabilities.

Taking the example of a task scheduler application, the microkernel contains all the logic for scheduling and triggering tasks, while the plug-ins contain specific tasks. As long as the plug-ins adhere to a predefined API, the microkernel can trigger them without having to know the implementation details.

**Usage:**

- Applications that have a clear segmentation between basic routines and higher-order rules.
- Applications that have a fixed set of core routines and dynamic set of rule that needs frequent updates.

**Shortcoming:**

- The plugins must have good handshaking code so that the microkernel is aware of the plugin installation and is ready to work.
- Changing a microkernel is almost impossible if there are multiple plugins dependent on it.
- It is difficult to choose the right granularity for the kernel function in advance and more complex at a later stage.
## Microservices Architecture Pattern

![](img/Business-Centric%20architectures%20and%20testability/microservices-architecture-pattern.png)

Microservices architecture pattern is seen as a viable alternative to monolithic applications and service-oriented architectures. The components are deployed as separate units through an effective, streamlined delivery pipeline. The pattern’s benefits are enhanced scalability and a high degree of decoupling within the application.

Owing to its decoupled and independent characteristics, the components are accessed through a remote access protocol. Moreover, the same components can be separately developed, deployed, and tested without interdependency on any other service component.

Netflix is one of the early adopters of the microservice architecture pattern. The architecture allowed the engineering team to work in small teams responsible for the end-to-end development of hundreds of microservices. These microservices work together to stream digital entertainment to millions of Netflix customers every day.

**Usage:**

- Businesses and web applications that require rapid development.
- Websites with small components, data centers with well-defined boundaries, and remote teams globally.

**Shortcoming:**

- Designing the right level of granularity for a service component is always a challenge.
- All applications do not include tasks that can be split into independent units.
- Performance can be affected because of tasks being spread across different microservices.
## Space-Based Architecture Pattern

![](img/Business-Centric%20architectures%20and%20testability/space-based-architecture-pattern.png)

The concept of tuple space – the idea of distributed shared memory is the basis of the name of this architecture. The space-based pattern comprises two primary components – a processing unit and a virtualized middleware.

The processing unit contains portions of application components, including web-based components and backend business logic. While smaller web applications could be deployed in a single processing unit, the larger applications could split the application functionality into multiple processing units to avoid functional collapse. Furthermore, the virtualized-middleware component contains elements that control various aspects of data synchronization and request handling. They can be custom-written or can be purchased as third-party products.

A bidding auction site can be considered as a fitting example for this architecture pattern. It functions as the site receives bids from internet users through a browser request. On receiving the request, the site records that bid with a timestamp, updates the information of the latest bid, and sends the data back to the browser.

**Usage:**

- Applications and software systems that function with a large user base and a constant load of requests.
- Applications that are supposed to address scalability and concurrency issues.

**Shortcoming:**

- It is a complex task to cache the data for speed without disturbing multiple copies.
## Client-Server Architecture Pattern

A client-server architecture pattern is described as a distributed application structure having two main components –  a client and a server. This architecture facilitates the communication between the client and the server, which may or may not be under the same network. A client requests specific resources to be fetched from the server, which might be in the form of data, content, services, files, etc. The server identifies the requests made and responds to the client appropriately by sending over the requested resources.

![](img/Business-Centric%20architectures%20and%20testability/client-server-architecture-pattern.png)

The functional characteristics of a client and a server is an example of programs that interact with one another within an application. The functionality of this architecture is highly flexible as a single server can serve multiple clients, or a single client can use multiple servers. The servers can be classified by the services or resources they provide, irrespective of how they perform.

Email is a prominent example of a model that is built using the client-server pattern. When a user/client searches for a particular email, the server looks into the pool of resources and sends the requested email resource back to the user/client. This also helps you to improve the user experience.

**Usage:**

- Applications like emails, online banking services, the World Wide Web, network printing, file sharing applications, gaming apps, etc.
- Applications that focus on real-time services like telecommunication apps are built with a distributed application structure.
- Applications that require controlled access and offer multiple services for a large number of distributed clients.
- An application with centralized resources and services that has to be distributed over multiple servers.

**Shortcomings:**

- Incompatible server capacity can slow down, causing a performance bottleneck.
- Servers are usually prone to a single point of failure.
- Changing the pattern is a complex and expensive process.
- Server maintenance can be a demanding and expensive task.
## Master-Slave Architecture Pattern

![](img/Business-Centric%20architectures%20and%20testability/master-slave-architecture-pattern.png)

Imagine a single database receiving multiple similar requests at the same time. Naturally, processing every single request at the same time can complicate and slow down the application process. A solution to this problem is a master-slave architecture pattern that functions with the master database launching multiple slave components to process those requests quickly.

As the title suggests, the master-slave architecture pattern can be pictured as a master distributing tasks to its slaves. Once the slave components finish their tasks, the distributed tasks are compiled by the master and displayed as the result.

One must note that the master has absolute control and power over the slave components, determining their communication and functional priorities. What makes this pattern unique is that each slave would process the requests simultaneously, providing the results at the same time. This also means that the slave operations would not be considered complete until every slave has returned the result to the master.

This pattern is well-suited for applications that can be divided into smaller segments for executing similar requests. An appropriate example would be a database application that requires heavy multitasking as its vital component.

**Usage:**

- Development of Operating Systems that may require a multiprocessors compatible architecture.
- Advanced applications where larger services have to be decomposed into smaller components.
- Applications processing raw data stored in different servers over a distributed network.
- Web browsers that follow multithreading to increase its responsiveness.

**Shortcomings:**

- Failure of the master component can lead to a loss of data with no backup over the slave components.
- Dependencies within the system can lead to a failure of the slave components.
- There can be an increase in overhead costs due to the isolated nature of the slave components.
## Pipe-Filter Architecture Pattern

![](img/Business-Centric%20architectures%20and%20testability/pipe-filter-architecture-pattern.png)

A pipe-filter architecture pattern processes a stream of data in a unidirectional flow where components are referred to as filters, and pipes are those which connect these filters. The chain of processing data takes place where the pipes transmit data to the filters, and the result of one filter becomes the input for the next filter. The function of this architecture is to break down significant components/processes into independent and multiple components that can be processed simultaneously.

The pipe-filter pattern is best suited for applications that process data in a stream using web services and can create simple sequences to complex structures. Compilers can be considered a fitting example having this architecture pattern since each filter performs lexical analysis, parsing, semantic analysis, and code generation.

**Usage:**

- It can be used for applications facilitating a simple, one-way data processing and transformation.
- Applications using tools like Electronic Data Interchange and External Dynamic List.
- Development of data compilers used for error-checking and syntax analysis.
- To perform advanced operations in Operating Systems like UNIX, where the output and input of programs are connected in a sequence.

**Shortcomings:**

- There can be a loss of data in between filters if the infrastructure design is not reliable.
- The slowest filter limits the performance and efficiency of the entire architecture.
- During transmission between filters, the data-transformation overhead costs might increase.
- The continuous transformational character of the architecture makes it less user-friendly for interactional systems.

## Broker Architecture Pattern

![](img/Business-Centric%20architectures%20and%20testability/broker-architecture-pattern.png)

A broker pattern is used for structuring distributed systems with decoupled components. By invoking remote services, components can interact with others in broker architecture patterns. Also, the broker is responsible for all the coordination and communication among the components.

Clients, servers, and brokers are three major components of the broker pattern. Generally, a broker will have access to all the services and characteristics related to a particular server. When clients request a service from the broker, the broker redirects them to a suitable service category for further process.

One of the key benefits of this architecture pattern is how it manages operations, such as change, addition, deletion, or relocation, related to objects in a dynamic manner. Lastly, this architecture pattern separates all communication-related code into layers from the application, allowing applications to run on distributed or single computers. Because of such advantages, broker architecture has been prevalent.

**Usage:**

- Used in message broker softwares such as Apache ActiveMQ, Apache Kafka, RabbitMQ, and JBoss Messaging.
- For structuring distributed systems that have decoupled components.

**Shortcomings:**

- Shallow fault tolerance capacity.
- Requires standardization of service description.
- The hidden layer may decrease software performance.
- Higher latency and requires more effort in deployment.
## Peer-to-Peer Architecture Pattern

![](img/Business-Centric%20architectures%20and%20testability/peer-to-peer-architecture-pattern.png)

In the peer-to-peer architectural pattern, individual components are called peers. A peer can act as a client, a server, or both and change its role dynamically over time. As a client, a peer can request service from other peers, and as a server, a peer can provide services to other peers. The significant difference between peer-to-peer and client-server architecture is that each computer on the network has considerable authority and the absence of a centralized server. Its capacity increases as more and more computers join the network.

An excellent example of a peer-to-peer architecture pattern would be file-sharing networks like Skype, BitTorrent, and Napster. In BitTorrent, peer-to-peer architecture is used for distributing the data and files on the internet in a decentralized fashion. By using this protocol, one can transfer large video and audio files with the utmost ease. In Skype, you use the VoIP P2P architecture pattern to make a voice call and send text messages to another user. In this manner, you can use peer-to-peer architecture for file sharing, messaging, collaboration, etc.

**Usage:**

- File-sharing networks such as Gnutella and G2.
- Cryptocurrency-based products such as Bitcoin and Blockchain.
- Multimedia products such as P2PTV and PDTP.

**Shortcomings:**

- No guarantee of high-quality service.
- Achieving robust security is challenging.
- Performance depends on the number of nodes connected to the network.
- No way to backup files or folders.
- Might need a specific interface to read the file.

# Domain Driven Design
**Domain-driven design** (**DDD**) is a major software design approach, focusing on modeling software to match a domain according to input from that domain's experts.[\[2\]](https://en.wikipedia.org/wiki/Domain-driven_design#cite_note-vernon2013-2)

Under domain-driven design, the structure and language of software code (class names, class methods, class variables) should match the business domain. For example, if software processes loan applications, it might have classes like LoanApplication and Customer, and methods such as AcceptOffer and Withdraw.

Domain-driven design is predicated on the following goals:

- placing the project's primary focus on the core domain and domain logic;
- basing complex designs on a model of the domain;
- initiating a creative collaboration between technical and domain experts to iteratively refine a conceptual model that addresses particular domain problems.
## **Isolating the domain: the layered architecture**

![](img/Business-Centric%20architectures%20and%20testability/ddd-layers.png)

Domain-Driven Design focuses on domain modeling, and separating the model (or business logic) from the implementation details (e.g. which database we use).

Indeed, if the domain-related code is mixed with other code, it rapidly becomes very difficult to reason about.

The recommended architecture is made of 4 layers.

### User Interface (or Presentation Layer)
Responsible for showing information to the user and interpreting the user’s commands. The external actor might sometimes be another computer system rather than a human user.
### Application Layer
Defines the jobs (use cases) the software is supposed to do and coordinates the domain objects to work out problems.

This layer is kept thin. It does not contain business rules or knowledge, but only coordinates tasks and delegates work to collaborations of domain objects in the next layer down.

It does not have state reflecting the business situation, but it can have state that reflects the progress of a task for the user or the program.
### Domain Layer (or Model Layer)
Responsible for representing concepts of the business, information about the business situation, and business rules.

State that reflects the business situation is controlled and used here, even though the technical details of storing it are delegated to the infrastructure.

This layer is the heart of business software.
### Infrastructure Layer
Provides generic technical capabilities that support the higher layers: message sending for the application, persistence for the domain, drawing widgets for the UI, and so on. The infrastructure layer may also support the pattern of interactions between the four layers through an architectural framework.
### Domain modeling
Domain modeling is the activity of describing the domain knowledge with concepts and structures that will help reason about the domain and implement it.

The basic constraint is that the model must both **help the implementation of features** and **represent real-life knowledge**.

To enforce the representation of the domain inside the code, Domain-Driven Design also encourages the use of an “*Ubiquitous Language”*, which is shared between developers and business people.
### Best practices to enforce healthy domain modeling
- Use the model as the backbone of the Ubiquitous Language.
- Commit the team to exercising that language relentlessly in all communication within the team and in the code.
- Use the same language in diagrams, writing, and especially speech.
- Resolve confusion over terms in conversation, in just the way we come to agree on the meaning of ordinary words.
- When changes are made to the model, refactor the code (renaming classes, methods, modules, …) to conform to the new model.
- Recognize that a change in the Ubiquitous Language is a also change to the model.
- Conversely, developers need to realize that changing the code also means changing the model.
- Domain experts (product people) should object to terms or structures that are awkward or inadequate to convey domain understanding; developers should watch for ambiguity or inconsistency that will trip up design.

## **Expressing the model: Building Blocks**
There are 3 tools to express the model in Domain-Driven Design, which can be grouped in Modules:

- Value Objects
- Entities
- Services

### Value objects
Value Objects are simple objects that convey meaning and functionality. These objects describe things but don’t have a special identity.

Value Objects are often passed as parameters in messages between objects. They are frequently temporary created for an operation and then discarded.

**Best practices:**

- Treat the Value Object as immutable.
- Don’t give it any identity and avoid the design complexities necessary to maintain Entities.
- Ensure that the attributes that make up a Value Object form a conceptual whole.

### Entities
An Entity is an object defined primarily by its identity, rather than specific attributes. The identity of an Entity runs through time, and possibly different representations. Entities are also called “reference objects”.

### Services
In some cases, the clearest and most pragmatic design includes operations that do not conceptually belong to any object. Rather than force the issue, we can follow the natural contours of the problem space and include Services explicitly in the model.

### Modules
The Modules in the domain layer should emerge as a meaningful part of the model, telling the story of the domain on a larger scale.

There should be low coupling between Modules and high cohesion within them, both code-wise and concept-wise:

- There is a limit to how many things a person can think about at once (hence low coupling).
- Incoherent fragments of ideas are as hard to understand as an undifferentiated soup of ideas (hence high cohesion).

**Best practices:**

- Give the Modules names that become part of the Ubiquitous Language. Modules and their names should reflect insight into the domain.
- When creating modules, favor conceptual clarity over technical convenience (if both are not achievable together).

## Managing the life cycle of domain objects
The goal is to prevent the model from getting swamped by the complexity of managing the life cycle. To do this, we separate the management of the life cycle (i.e. persisting objects) from the business logic.

The most important concepts for this are Aggregates and Repositories. Note: an Aggregate is always associated with one and only one Repository.
### Aggregates
Aggregates are a cluster of Entities and Value Objects that make sense domain-wise and are retrieved and persisted together.

Aggregates add structure to the model by setting boundaries and providing a clear ownership for the objects they contain.

**Best practices:**

- Cluster the Entities and Value Objects into Aggregates and define boundaries around each.
- Choose one Entity to be the root of each Aggregate, and control all access to the objects inside the boundary through the root.
- Allow external objects to hold references to the root only. This arrangement makes it practical to enforce all invariants for objects in the Aggregate and for the Aggregate as a whole in any state change.

Even though aggregates help manage the life cycle by defining ownership and boundaries, they know nothing about the details of the infrastructure and belong in the domain layer.
### Repositories
Repositories offer an interface to retrieve and persist aggregates. They hide the database details from the domain. Repositories manage the middle and end of the life cycle.

Repository interfaces are **declared in the Domain Layer**, but the repositories themselves are **implemented in the Infrastructure Layer**. This makes it easy to switch between different implementations of a repository without impacting any business code (for instance going from SQL to No-SQL storage, or writing in-memory implementations for faster tests).
##
## Anti-corruption Layer pattern
Implement a façade or adapter layer between different subsystems that don't share the same semantics. This layer translates requests that one subsystem makes to the other subsystem. Use this pattern to ensure that an application's design is not limited by dependencies on outside subsystems. This pattern was first described by Eric Evans in *Domain-Driven Design*.

**Context and problem**

Most applications rely on other systems for some data or functionality. For example, when a legacy application is migrated to a modern system, it may still need existing legacy resources. New features must be able to call the legacy system. This is especially true of gradual migrations, where different features of a larger application are moved to a modern system over time.

Often these legacy systems suffer from quality issues such as convoluted data schemas or obsolete APIs. The features and technologies used in legacy systems can vary widely from more modern systems. To interoperate with the legacy system, the new application may need to support outdated infrastructure, protocols, data models, APIs, or other features that you wouldn't otherwise put into a modern application.

Maintaining access between new and legacy systems can force the new system to adhere to at least some of the legacy system's APIs or other semantics. When these legacy features have quality issues, supporting them "corrupts" what might otherwise be a cleanly designed modern application.

Similar issues can arise with any external system that your development team doesn't control, not just legacy systems.

**Solution**

Isolate the different subsystems by placing an anti-corruption layer between them. This layer translates communications between the two systems, allowing one system to remain unchanged while the other can avoid compromising its design and technological approach.

![](img/Business-Centric%20architectures%20and%20testability/anti-corruption-layer-pattern.png)

The diagram above shows an application with two subsystems. Subsystem A calls to subsystem B through an anti-corruption layer. Communication between subsystem A and the anti-corruption layer always uses the data model and architecture of subsystem A. Calls from the anti-corruption layer to subsystem B conform to that subsystem's data model or methods. The anti-corruption layer contains all of the logic necessary to translate between the two systems. The layer can be implemented as a component within the application or as an independent service.

**Issues and considerations**

- The anti-corruption layer may add latency to calls made between the two systems.
- The anti-corruption layer adds an additional service that must be managed and maintained.
- Consider how your anti-corruption layer will scale.
- Consider whether you need more than one anti-corruption layer. You may want to decompose functionality into multiple services using different technologies or languages, or there may be other reasons to partition the anti-corruption layer.
- Consider how the anti-corruption layer will be managed in relation with your other applications or services. How will it be integrated into your monitoring, release, and configuration processes?
- Make sure transaction and data consistency are maintained and can be monitored.
- Consider whether the anti-corruption layer needs to handle all communication between different subsystems, or just a subset of features.
- If the anti-corruption layer is part of an application migration strategy, consider whether it will be permanent, or will be retired after all legacy functionality has been migrated.

**When to use this pattern**

Use this pattern when:

- A migration is planned to happen over multiple stages, but integration between new and legacy systems needs to be maintained.
- Two or more subsystems have different semantics, but still need to communicate.

This pattern may not be suitable if there are no significant semantic differences between new and legacy systems.

# Ports and Adapters Architecture

![](img/Business-Centric%20architectures%20and%20testability/hexagonal-architecture.png)

The **ports and adapters architecture**, or **hexagonal architecture**, is an architectural pattern used in software design. It aims at creating loosely coupled application components that can be easily connected to their software environment by means of ports and adapters. This makes components exchangeable at any level and facilitates test automation.

The hexagonal architecture divides a system into several loosely-coupled interchangeable components, such as the application core, the database, the user interface, test scripts and interfaces with other systems. This approach is an alternative to the traditional layered architecture.

![](img/Business-Centric%20architectures%20and%20testability/ports.png)

## Ports
Each component is connected to the others through a number of exposed "ports". Communication through these ports follow a given protocol depending on their purpose. Ports and protocols define an abstract API that can be implemented by any suitable technical means (e.g. method invocation in an object-oriented language, remote procedure calls, or Web services). We can see a Port as a technology-agnostic entry point, it determines the interface which will allow foreign actors to communicate with the Application, regardless of who or what will implement said interface. Just as a USB port allows multiple types of devices to communicate with a computer as long as they have a USB adapter. Ports also allow the Application to communicate with external systems or services, such as databases, message brokers, other applications, etc.

The granularity of the ports and their number is not constrained:

- a single port could in some case be sufficient (e.g. in the case of a simple service consumer) ;
- typically, there are ports for event sources (user interface, automatic feeding), notifications (outgoing notifications), database (in order to interface the component with any suitable DBMS), and administration (for controlling the component);
- In an extreme case, there could be a different port for every use case, if needed.
## Adapters

![](img/Business-Centric%20architectures%20and%20testability/adapters.png)

Adapters are the glue between components and the outside world. They tailor the exchanges between the external world and the ports that represent the requirements of the inside of the application component. There can be several adapters for one port, for example, data can be provided by a user through a GUI or a command-line interface, by an automated data source, or by test scripts. An Adapter will initiate the interaction with the Application through a Port, using a specific technology, for example, a REST controller would represent an adapter that allows a client to communicate with the Application. There can be as many Adapters for any single Port as needed without this representing a risk to the Ports or the Application itself.
## Application
The Application is the core of the system, it contains the Application Services which orchestrate the functionality or the use cases. It also contains the Domain Model, which is the business logic embedded in Aggregates, Entities, and Value Objects. The Application is represented by a hexagon which receives commands or queries from the Ports, and sends requests out to other external actors, like databases, via Ports as well.
## The Dependency Rule

![](img/Business-Centric%20architectures%20and%20testability/dependency-rule.png)

*Source code dependencies* can only point *inwards*. Nothing in an inner circle can know anything at all about something in an outer circle.
## Why use ports and adapters architecture?
There are many advantages of using the Ports and Adapters Architecture, one of them is to be able to completely isolate your application logic and domain logic in a fully testable way. Since it does not depend on external factors, testing it becomes natural and mocking its dependencies is easy.

It also lets you design all your system’s interfaces ‘by purpose’ rather than by technology, preventing you from lock-in, and making it easier for your application’s tech stack to evolve with time. If you need to change the persistence layer, go for it. If you need to allow your app to be called by Slack bots instead of humans, you got it! All you need to do is implement new Adapters and you’re good to go.

The Ports and Adapters architecture also plays along very well with Domain-Driven Design, the main advantage it brings is that it shields domain logic to leak out of your application’s core. Just be vigilant of leakage between Application and Domain layers.

This architecture is not appropriate for small websites.  It is appropriate for long-lived business applications as well as applications with complex behavior.  It emphasizes the use of interfaces for behavior contracts, and it forces the externalization of infrastructure.  The diagram you see here is a representation of traditional layered architecture.   This is the basic architecture that is most frequently used.  Each subsequent layer depends on the layers beneath it, and then every layer normally will depend on some common infrastructure and utility services.  The big drawback to this top-down layered architecture is the coupling that it creates.  Each layer is coupled to the layers below it, and each layer is often coupled to various infrastructure concerns.  However, without coupling, our systems wouldn’t do anything useful, but this architecture creates unnecessary coupling. The biggest offender (and most common) is the coupling of UI and business logic to data access.  Yes, UI is coupled to data access with this approach.  Transitive dependencies are still dependencies.  The UI can’t function if business logic isn’t there.  Business logic can’t function if data access isn’t there. Data access changes frequently.  Historically, the industry has modified data access techniques at least every three years; therefore, we can count on needing to modify data access three years from now for any healthy, long-lived systems that’s mission-critical to the business.  We often don’t keep systems up-to-date because it’s impossible to do.  If coupling prevents easily upgrading parts of the system, then the business has no choice but to let the system fall behind into a state of disrepair. This is how legacy systems become stale, and eventually they are rewritten.

![](img/Business-Centric%20architectures%20and%20testability/traditional-layered-architecture.png)

# Clean Architecture

![](img/Business-Centric%20architectures%20and%20testability/clean-architecture.png)

1. Independent of Frameworks. The architecture does not depend on the existence of some library of feature laden software. This allows you to use such frameworks as tools, rather than having to cram your system into their limited constraints.
1. Testable. The business rules can be tested without the UI, Database, Web Server, or any other external element.
1. Independent of UI. The UI can change easily, without changing the rest of the system. A Web UI could be replaced with a console UI, for example, without changing the business rules.
1. Independent of Database. You can swap out Oracle or SQL Server, for Mongo, BigTable, CouchDB, or something else. Your business rules are not bound to the database.
1. Independent of any external agency. In fact your business rules simply don’t know anything at all about the outside world.

## **The Dependency Rule**
The concentric circles represent different areas of software. In general, the further in you go, the higher level the software becomes. The outer circles are mechanisms. The inner circles are policies.

The overriding rule that makes this architecture work is *The Dependency Rule*. This rule says that *source code dependencies* can only point *inwards*. Nothing in an inner circle can know anything at all about something in an outer circle. In particular, the name of something declared in an outer circle must not be mentioned by the code in an inner circle. That includes functions, classes. variables, or any other named software entity.

By the same token, data formats used in an outer circle should not be used by an inner circle, especially if those formats are generated by a framework in an outer circle. We don’t want anything in an outer circle to impact the inner circles.

## ***Entities***
Entities encapsulate *Enterprise wide* business rules. An entity can be an object with methods, or it can be a set of data structures and functions. It doesn’t matter so long as the entities could be used by many different applications in the enterprise.

If you don’t have an enterprise, and are just writing a single application, then these entities are the business objects of the application. They encapsulate the most general and high-level rules. They are the least likely to change when something external changes. For example, you would not expect these objects to be affected by a change to page navigation, or security. No operational change to any particular application should affect the entity layer.

## **Use Cases**
The software in this layer contains *application specific* business rules. It encapsulates and implements all of the use cases of the system. These use cases orchestrate the flow of data to and from the entities, and direct those entities to use their *enterprise wide* business rules to achieve the goals of the use case.

We do not expect changes in this layer to affect the entities. We also do not expect this layer to be affected by changes to externalities such as the database, the UI, or any of the common frameworks. This layer is isolated from such concerns.

We *do*, however, expect that changes to the operation of the application *will* affect the use-cases and therefore the software in this layer. If the details of a use-case change, then some code in this layer will certainly be affected.

## **Interface Adapters**
The software in this layer is a set of adapters that convert data from the format most convenient for the use cases and entities, to the format most convenient for some external agency such as the Database or the Web. It is this layer, for example, that will wholly contain the MVC architecture of a GUI. The Presenters, Views, and Controllers all belong here. The models are likely just data structures that are passed from the controllers to the use cases, and then back from the use cases to the presenters and views.

Similarly, data is converted, in this layer, from the form most convenient for entities and use cases, into the form most convenient for whatever persistence framework is being used. i.e. The Database. No code inward of this circle should know anything at all about the database. If the database is a SQL database, then all the SQL should be restricted to this layer, and in particular to the parts of this layer that have to do with the database.

Also in this layer is any other adapter necessary to convert data from some external form, such as an external service, to the internal form used by the use cases and entities.

## **Frameworks and Drivers.**
The outermost layer is generally composed of frameworks and tools such as the Database, the Web Framework, etc. Generally you don’t write much code in this layer other than glue code that communicates to the next circle inwards.

This layer is where all the details go. The Web is a detail. The database is a detail. We keep these things on the outside where they can do little harm.

## **Only Four Circles?**
No, the circles are schematic. You may find that you need more than just these four. There’s no rule that says you must always have just these four. However, *The Dependency Rule* always applies. Source code dependencies always point inwards. As you move inwards the level of abstraction increases. The outermost circle is low level concrete detail. As you move inwards the software grows more abstract, and encapsulates higher level policies. The innermost circle is the most general.

## **Crossing boundaries.**
At the lower right of the diagram is an example of how we cross the circle boundaries. It shows the Controllers and Presenters communicating with the Use Cases in the next layer. Note the flow of control. It begins in the controller, moves through the use case, and then winds up executing in the presenter. Note also the source code dependencies. Each one of them points inwards towards the use cases.

We usually resolve this apparent contradiction by using the Dependency Inversion Principle. In a language like Java, for example, we would arrange interfaces and inheritance relationships such that the source code dependencies oppose the flow of control at just the right points across the boundary.

For example, consider that the use case needs to call the presenter. However, this call must not be direct because that would violate *The Dependency Rule*: No name in an outer circle can be mentioned by an inner circle. So we have the use case call an interface (Shown here as Use Case Output Port) in the inner circle, and have the presenter in the outer circle implement it.

The same technique is used to cross all the boundaries in the architectures. We take advantage of dynamic polymorphism to create source code dependencies that oppose the flow of control so that we can conform to *The Dependency Rule* no matter what direction the flow of control is going in.

## **What data crosses the boundaries.**
Typically the data that crosses the boundaries is simple data structures. You can use basic structs or simple Data Transfer objects if you like. Or the data can simply be arguments in function calls. Or you can pack it into a hashmap, or construct it into an object. The important thing is that isolated, simple, data structures are passed across the boundaries. We don’t want to cheat and pass *Entities* or Database rows. We don’t want the data structures to have any kind of dependency that violates *The Dependency Rule*.

For example, many database frameworks return a convenient data format in response to a query. We might call this a RowStructure. We don’t want to pass that row structure inwards across a boundary. That would violate *The Dependency Rule* because it would force an inner circle to know something about an outer circle.

So when we pass data across a boundary, it is always in the form that is most convenient for the inner circle.


# Testing Strategy for Hexagonal Applications

## Unit testing

A test is not a unit test if:

- Talks to the database
- Communicates across the network
- Touches the file system
- Can’t run at the same time as any of your other unit tests

*Tests that do these things aren't bad. Often they are worth writing, and they can be written in a unit test harness. However, it is important to be able to separate them from true unit tests so that we can keep a set of tests that we can run fast whenever we make our changes.*

That might sound a little severe, but it is medicine for a common problem. Generally, unit tests are supposed to be small, they test a method or the interaction of a couple of methods. When you pull the database, sockets, or file system access into your unit tests, they aren't really about those methods any more; they are about the integration of your code with that other software. If you write code in a way which separates your logic from OS and vendor services, you not only get faster unit tests, you get a binary chop that allows you to discover whether the problem is in your logic or in the things you are interfacing with. If all the unit tests pass but the other tests (the ones not using mocks) dont, you are far closer to isolating the problem.

When using the Hexagonal architecture, you can easily write unit tests that are fast and follow the definition of a unit test, for the domain of your application. That’s because domain shouldn’t have access to:

- Framework
- Controller action
- Database
- External services
- Filesystem
- System clock
- Randomness

## Integration testing

To check that the code works in production, we should prove that the port adapters implement

these contracts correctly. Adapter tests are called integration tests.

Integration tests:

- Test a repository against the actual database
- Test an HTTP integration against the real web service
- Test an application service using a real web controller

## System testing
Also, we need to check that the deployable unit (fully integrated software product) works. There are system tests for this.

**System Testing** is a level of testing that validates the complete and fully integrated software product. The purpose of a system test is to evaluate the end-to-end system specifications. Usually, the software is only one element of a larger computer-based system. Ultimately, the software is interfaced with other software/hardware systems. System Testing is defined as a series of different tests whose sole purpose is to exercise the full computer-based system.

## Acceptance testing

An **acceptance test** is a formal description of the behavior of a software product, generally expressed as an example or a usage scenario.

Difference between System testing and Acceptance testing.

**System Testing** is done to check whether the software or product meets the specified requirements or not. It is done by both testers and developers. It contains the Testings: System testing, Integration Testing. It is done through more positive and negative test cases.

**Acceptance Testing** is done after the system testing. It is used to check whether the software meets the customer requirements or not. Acceptance testing is used by testers, stakeholders as well as clients.

![](img/Business-Centric%20architectures%20and%20testability/testing-pyramid.png)

In order to build a well tested product, you need to use all of these.

## Prefer unit tests

Difficulty in writing unit tests is a reliable indicator of poor architecture design and code smell. The number of tests that are not isolated checks is a reliable metric of the quality of code. More integrated tests encourage us to be less careful with design and have worse architecture - negative feedback loop. **Write unit tests instead of avoiding listening to design pressure.**

![](img/Business-Centric%20architectures%20and%20testability/prefer-unit-tests-pyramid.png)

The higher we go up in our test pyramid, the higher the costs. This effect is multiplied because the cost increases involve both:

- Setting up the tests
- Maintenance of tests in place
- The cost of executing the tests (machine time, servers…)
- The time it takes to obtain a test result
- Analysis time when a bug is detected

These are all good reasons to invest in the deepest layers to optimize the [ROI](https://en.wikipedia.org/wiki/Return_on_investment) of our testing process. Each stage will complement the tests already passed, with a lower test volume. Each test level has its importance and is essential to ensure the level of quality we want for our customers, but we have to consciously evaluate the investments at all levels so as not to be overtaken by the induced costs which can easily escalate.

The graph below also illustrates the costs and time required to achieve status throughout the development cycle. This is another good reason to invest in unit testing.

![](img/Business-Centric%20architectures%20and%20testability/length-of-feedback-cycle.png)


