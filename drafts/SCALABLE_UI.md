# Challenges for Component Libraries

1) Maintainability

It is the deferment of labor

2) WHAT DO

I'd like to point out that other than a few compound aria patterns, the browser gives you everything you're going to need to render perfectly accessible

So the first thing you should do is nothing. Just use vanilla components. A checkbox is a checkbox.

Build what you can using entirely vanilly interactive components. You might not like it but you know who will? anyone who uses a keyboard. Or a mouse. You wont have to relay state from some 2mb bullshit "checkbox" web component. You won't have to learn any fucking SASS apis. Your cohort won't need to mind meld to arbitrary css taxonomy. It's JUST a checkbox.



3) OMFG I HIRED CARGO CULT DEVS

Yah you fucking did. You wanted to get an application done but now you're forever at 80% complete. You just bonused someone for making sure all buttons have the consistent blue. Bet that fucking hurt man.

You wanted a frontend dev. But everyone kept screaming reactjs and nextjs. Now you got some economics major turned coder that can't code shit except spaghetti react components and keeps trying to integrate whatever the next fucking tailwind is. And when you ask why you don't get reassurance aboout all the past sunk cost. You get to hear the same stupid mantra from a 27 year old who only works here to go skiing.

4) SECURITY

By utilizing CSS only changes and interacting with the least amount of JS, the surface area of bad faith actions is reduces. IE less javascript is less room to mess up.

If youre loading external components from cross-origin urls in production client level I'm going to ask you to quit and go back to your economics degree from UCSB.

4) SCAlABILITY

Most developers wouldnt associate "scalability" with component sets.

The hidden cost of deployment

First consider three scenarios where someone would use a component library.
- a complex enough app (JS async load cost is less than HTML cost)
- "recognition" across several scoped apps. (ie mutliple internal and external products)
- 
A universal component library will break all screenshot tests on any visual change. Including simple and ubiquitous browser renders like box shadows and decompression and image processing.

Most component libraries are entirely coupled to a framework if not a specific combo of frameworks that mandate structure.

How is that even useful??

5) The SOLUTION?

Easy solve
Any component library CANNOT be directly tired to a monolith repository or a multi-product.
As in, a one version component library, is simply not sustainable.
I know that's not what you want to hear.
But whatever developer is selling that engineering dream to you is a fucking idiot.
Probably read books about "beautiful" code and shit.
Sorry.
Sorry that's not what engineering is. Let alone a computer science engineer.


