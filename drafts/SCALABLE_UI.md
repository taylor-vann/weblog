# Challenges for Component Libraries

A) 

B)

C)


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

4) SCALABILITY

Most developers wouldnt associate "scalability" with component sets.

The hidden cost of deployment

First consider three scenarios where someone would use a component library.
- a complex enough app (JS async load cost is less than HTML cost)
- "recognition" across several scoped apps. (ie mutliple internal and external products)
- 
A universal component library will break all screenshot tests on any visual change. Including simple and ubiquitous browser renders like box shadows and decompression and image processing.

*** 
Imagine dozens of apps at a workspace

All branded the same

Using the same repository of components.

Great! You and your team must feel special. Hope you feel fucking special. I bet you got a bonus too huh?

But me, the engineer hired years later to clean up your mess?

I have to justify why it took 6 months to update one line of css that broke every screenshot in every fucking company project.

And I can't give realistic answers to manager like: the previous engineers took you for a ride and sold you on the integration equivalent of a "wall of fire".

And then my manager goes, BTW here's YOUR bonus: "middle finger emoji lolol".


## Right skewed component usage
***

Your component usage is perpetually right skewed.

We think "component set" makes -> an "app".

And we assume there will be forms and buttons and media and input elements.

And that can be true but closer inspection will reveal that not a single project uses every single component, that most teams don't use more than 4-5 components.

All that planning.
All that designing.language
All that fucking budget money POOF.
For a buttons and a custom checkbox.

And that's at the BEST and MOST SUCCESSFUL case.

There are tiers of failure.

I've listed the following in progressing intensiveness

- Components are made entirely in one framework
- The components aren't even aria accessible



Most component libraries are entirely coupled to a framework if not a specific combo of frameworks that mandate structure.

How is that even useful??

5) The SOLUTION?

Easy solve
Any component library CANNOT be directly tired to a monolith repository or a multi-product.
As in, a one version component library, is simply not feasible or sustainable.
I know that's not what you want to hear.
But whatever developer is selling that engineering dream to you is a fucking idiot.
Probably read books about "beautiful" code and shit.
Sorry.
Sorry that's not what engineering is. Let alone a computer science engineer.
It's work

Because here's what's gonna happen.
Your component set, will not be perfect out the gate.
It will need updates just like the rest of our software.
It will have bugs just like the rest of our software.
Which means downstream apps will work but not perfectly.
You'll update.
But 50 teams need to adopt the update.
They're in charge of code in their repos.
They're pinged with the change.
49 people will say yes.
1 person will challenge the update.
Screenshots broke.
Rollback they say.
ANd by the time you placate that one asshole engineer turned product manager, 25 of the other teams have submitted updates.
You need to get 25 more approvals.
Fuck.
That button you were updating? Is the most common component across your company.
It breaks 78,432 screenshots. This could indicate a breaking change.
We need to run this as an experiment.
Release in powers of 10 across the repo.
Since this is an infrastructure issure contact the Core team.
Theyve approved three weeks later.
Reach out to these 50 managers and have their approval for this experiment.
Experiments take up to 1month to 6months to complete.

SO DON"T SIT HERE AND FUCKING TELL ME UI COMPONENTS BELONG IN A GAWD DAMN MONOLITH

It's an engineering smell.
It smells green. Like that engineer hasn't had their tail whipped by their frontend hubris comes crashing down.

It tells me you've never encountered this problem and you have no fucking idea how to handle it.

You're an engineer that hasn't had their ass handed to them yet. Even worse if you're a team lead and you haven't had your ass handed to you yet. You sure your okay with risking all your teammates career over your stupid fucking button?

## The dirty secret about monoliths, monolith? more like monomyth

Let's break down the myth of monoliths in theory and in practice

Practice:

Do you think the "google" monolith contains only a single version of android? What about chrome?

No. They have every fucking version of android and chrome. And they publicly expose dozens.

So out of the gate we are breaking one version myths.

What about APIs? Surely that's the dream right? A backend that's one version everywhere?

Cool Cool.
So how do you upgrade downstream clients?

Oh you're gonna submit a request to change dozens of products outside of your team's purview?

I'd like to see any manager that lets people upstream wreck their forward facing product.

Oh youre gonna update them yourself?

That's not fucking happening.

You're gonna sit your little ass down and wait for the tech lead and manager to clear your changes. You'll schedule a 30 min meeting that baloons into several weeks of talks and expected changes.

So what's the solution?

An extemely slow and asyncronous swap.

Remember the classic "swap a variable" exercise you learned at Comp Sci school?

A = 3
b = 2

SO you need a C to swap

C = A

A = b
B = A

Now theyre swapped?

A very similar thing happends in a monolith.

You'll make an endpoint other projects can temporarily point to A. When it's time to upgrade, you'll create another endpoint B.

Slowly everyone will migrate to endpoint B.

Then once all clients are migrated to B you can:
remove A entirely (not usually possible)
or relable B as A.

BUt the catch is this is 2 versions in a monolith. As long as there is  There will always be AT LEAST two versions 

