---
title: Carousel
description: A slideshow component that cycles through content panels with previous/next navigation controls and optional auto-play.
component: true
---

## Usage

```bash
rust
use ui::{Carousel, CarouselContent, CarouselItem, CarouselNext, CarouselPrevious, CarouselIndicators};

<Carousel>
    <CarouselContent>
        <CarouselItem>
            <img src="https://picsum.photos/id/1" alt="Slide 1" class="w-full aspect-video object-cover" />
        </CarouselItem>
        <CarouselItem>
            <img src="https://picsum.photos/id/2" alt="Slide 2" class="w-full aspect-video object-cover" />
        </CarouselItem>
        <CarouselItem>
            <img src="https://picsum.photos/id/3" alt="Slide 3" class="w-full aspect-video object-cover" />
        </CarouselItem>
    </CarouselContent>
    <CarouselPrevious />
    <CarouselNext />
    <CarouselIndicators />
</Carousel>

```

## Props

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| orientation | CarouselOrientation | CarouselOrientation::Horizontal | The slide direction. |
| auto_play | bool | false | Whether to auto-advance slides. |
| auto_play_interval_ms | u64 | 5000 | Milliseconds between auto-advance. |
| class | String |  | Additional CSS classes to apply to the root element. |
```

## Examples

```bash
rust
use ui::{Carousel, CarouselContent, CarouselItem, CarouselNext, CarouselPrevious, CarouselIndicators};

<Carousel>
    <CarouselContent>
        <CarouselItem>
            <img src="https://picsum.photos/id/1" alt="Slide 1" class="w-full aspect-video object-cover" />
        </CarouselItem>
        <CarouselItem>
            <img src="https://picsum.photos/id/2" alt="Slide 2" class="w-full aspect-video object-cover" />
        </CarouselItem>
        <CarouselItem>
            <img src="https://picsum.photos/id/3" alt="Slide 3" class="w-full aspect-video object-cover" />
        </CarouselItem>
    </CarouselContent>
    <CarouselPrevious />
    <CarouselNext />
    <CarouselIndicators />
</Carousel>

```
