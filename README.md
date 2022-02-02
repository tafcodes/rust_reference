# Autonomous Intersection Experiments

I want to play around with some algorithms for pushing cars through an intersection
with an eye towards how throughput will change as a result of _coordination bandwidth_.

This is, in a vague sense, an information theory problem.  Or at least that's the
way I'm choosing to consider it.

## Coordination Bandwidth

"Coordination Bandwidth" is just the bits-per-tick of information which is shared
between parties.  It will never be zero, as the most basic algorithms will rely on
at least being able to "see" in space.  

If, for example, upon every tick every vehicle could scan (with imaginary LiDAR)
360 degrees with 1 sample per degree with 1 byte representing distance-to-object, then I would 
say that there's `360 * 1 byte` = *360 bytes* of coordination bandwidth per vehicle.

Now, suppose we added a "radio link" to every vehicle which, every tick, broadcasts
position and heading to every other vehicle (akin to ADS-B).  This would increase
the coordination bandwith by the size of these messages.

My intuition is that greater coordination bandwith will allow for higher intersection
throughput, to a point.  

While I am working on this project, I will be reading about Information Theory,
and I thought this would be a fun way to see things play out.  

## Implementation

I could do this all on my machine, but I thought it would be more fun to do it on
your machine.

So, web technologies.  I'm not a front-end guy.  Canvases it is.  I don't like 
JavaScript, so let's use WebAssembly and Rust.  There's trivial concurrency here,
so let's use WebWorkers.

I haven't seen it done, but I bet a WebWorker can be a wasm program.  

### Infrastructure

I'll use DigitalOcean and CloudFlare.  If I can come up with an excuse, I might use
Kubernetes.

## Future
It might be interesting to give other people a way to participate.  This is a fun
algorithmic space.  Maybe let people write their own algorithms and compete for 
max throughput under specific conditions?  All I'd need to do is allow uploading,
do some sandboxed serverside execution, and host a leaderboard with test results....

## The Present

I will be learning Rust as I go along, so this repo is going to have a long and varied history, rather than tracking just the simple implementation of a project.



