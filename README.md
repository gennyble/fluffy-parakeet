# Ludum Dare 50
Okay, okay. I've tried to do a few Ludum Dare in the recent past, but have always fallen short (except for two around the late 30's). I've got to remember to keep the scope small and get something vaguely game shaped.

I don't have an engine right now, just a bunch of dependencies. I've pre-added things I think I'll want to the Cargo.toml which is probably not cheating. People setup scene transitions and stuff, this has to be fine! I'm not really in it for the competitive aspect, anyway, so if I get like, disqualified I won't have really lost anything.

I've done my first ["I'm In" post](https://ldjam.com/events/ludum-dare/50/$279969/im-in) for some reason. It echos a lot of what's here. I think I'll use this readme as a sort of project log. I don't really feel like tweeting it and I'm not sure where it would fit in the [wikarden](https://nyble.dev/garden/home.html). It's my website, I could find space, but I think I'll just write a retrospective later, maybe.

## Day 0

20:02 - The theme is Delay The Inevitable and oh gosh. Okay, have to think of ideas. Time to shower!

20:51 - Showered forever, here are some ideas: *(do we not get GitHub style task buttons in readme?? `- []`)*

 - Winter campsite, you're alone. A huge blizzards happens and you have to delay your inevitable death. Manage your tiredness, food, and fire to stay alive. This is not my favorite.

 - The window is collapsing from one side, prevent it for as long as you can. You're a little character on a tile world of some sort- side scrolling or top-down. How do you prevent the window closing? I was thinking you wedge some stuff between it and some pre-existing level geometry. How does this stop working? What prevents you from keeping it open forever? Does pressure slowly build up and you barricades break? Is this fun?

 - A sinking submarine. You've had a breach and your fuel pumps have died. You have a backup pump, only one. Manage your sole pump to send water overboard and fuel into your generators. Your crew need rest, too.

 23:01  
 Okay! I have figured out my idea. I'm going to go with the second one, the window collapsing, and it'll be great. A sidescroller where you use things around the level to prop up the window. They all have a strength and you have to keep up as long as you can. Maybe there'll eventually be a way to raise it back up a little. A little bottle jack? Mmmmm.

 23:24  
 thank you for the star, I live off of small bursts of dopamine apparently *(narrator: she's been broken by social media)*

 23:50  
 Ahh okay! Did some planning in my notebook, I want to get a window up today and maybe even some stuff rendered on the screen. I'll probably pull some code out of [notsure](https://github.com/gennyble/notsure) but like, that's how I do a lot of programming, right? Reading past code instead of docs. This isn't not allowed, right? I don't care, goal to make a thing and I'm making a bloody thing! ^^

01:01 - f0fe373  
We have a window! Perhaps Rust was,,, not the best choice, but it'll be fun! It *has* been fun! Just a lot of boilerplate. Rust with more crates might be better.

03:34  
Burned some time I should've probably been working on this by responding to some emails and doing other things. Structured procrastination is a hell of a thing! Maybe I'll even finish my resume tomorrow like some sort of person that *does* things! As long as I get the game to a happy state, too, I think I'll be happy. It even has a title now, kind of. I'm calling it Collapse for the time being. It's set in the code *(only in like, two places)* so now it is forever.

## Day 1

17:20  
Heck! I slept forever and getting up is hard. I have half the compo to get this playable. Lots to do, lots to do~