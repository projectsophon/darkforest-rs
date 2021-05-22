use super::{ChunkFootprint, Coords};

/// A Spiral Iterator of ChunkFootprint
pub struct SpiralExplorer {
    chunk_side_length: u16,
    from_chunk: ChunkFootprint,
    current_chunk: ChunkFootprint,
}

impl SpiralExplorer {
    pub fn new(center: Coords, chunk_side_length: u16) -> Self {
        //floor by default?

        let length = i64::from(chunk_side_length);

        let bottom_left_x = (center.x / length) * length;
        let bottom_left_y = (center.y / length) * length;
        let bottom_left = Coords {
            x: bottom_left_x,
            y: bottom_left_y,
        };

        let from_chunk = ChunkFootprint {
            bottomLeft: bottom_left,
            sideLength: length,
        };

        Self {
            chunk_side_length,
            from_chunk: from_chunk.clone(),
            current_chunk: from_chunk,
        }
    }
}

impl Iterator for SpiralExplorer {
    type Item = ChunkFootprint;
    fn next(&mut self) -> Option<ChunkFootprint> {
        let Coords {
            x: home_x,
            y: home_y,
        } = self.from_chunk.bottomLeft;
        let Coords {
            x: curr_x,
            y: curr_y,
        } = self.current_chunk.bottomLeft;

        let mut next_bottom_left = self.current_chunk.bottomLeft.clone();

        let length = i64::from(self.chunk_side_length);

        if curr_x == home_x && curr_y == home_y {
            next_bottom_left.y = home_y + length;
        } else if curr_y - curr_x > home_y - home_x && curr_y + curr_x >= home_x + home_y {
            if curr_y + curr_x == home_x + home_y {
                // break the circle
                next_bottom_left.y = curr_y + length;
            } else {
                next_bottom_left.x = curr_x + length;
            }
        } else if curr_x + curr_y > home_x + home_y && curr_y - curr_x <= home_y - home_x {
            next_bottom_left.y = curr_y - length;
        } else if curr_x + curr_y <= home_x + home_y && curr_y - curr_x < home_y - home_x {
            next_bottom_left.x = curr_x - length;
        } else {
            // if (curr_x + curr_y < home_x + home_y && curr_y - curr_x >= home_y - home_x)
            next_bottom_left.y = curr_y + length;
        }

        self.current_chunk = ChunkFootprint {
            bottomLeft: next_bottom_left,
            sideLength: length,
        };
        Some(self.current_chunk.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sixteen_iter() {
        let center = Coords { x: 0, y: 0 };
        let chunk_side_length = 16;
        let mut explorer = SpiralExplorer::new(center, chunk_side_length);

        assert_eq!(
            explorer.next(),
            Some(ChunkFootprint {
                bottomLeft: Coords {
                    x: 0,
                    y: chunk_side_length as i64,
                },
                sideLength: chunk_side_length as i64
            })
        );

        assert_eq!(
            explorer.next(),
            Some(ChunkFootprint {
                bottomLeft: Coords {
                    x: chunk_side_length as i64,
                    y: chunk_side_length as i64,
                },
                sideLength: chunk_side_length as i64
            })
        );

        assert_eq!(
            explorer.next(),
            Some(ChunkFootprint {
                bottomLeft: Coords {
                    x: chunk_side_length as i64,
                    y: 0,
                },
                sideLength: chunk_side_length as i64
            })
        );

        assert_eq!(
            explorer.next(),
            Some(ChunkFootprint {
                bottomLeft: Coords {
                    x: chunk_side_length as i64,
                    y: -(chunk_side_length as i64),
                },
                sideLength: chunk_side_length as i64
            })
        );

        assert_eq!(
            explorer.next(),
            Some(ChunkFootprint {
                bottomLeft: Coords {
                    x: 0,
                    y: -(chunk_side_length as i64),
                },
                sideLength: chunk_side_length as i64
            })
        );
    }

    #[test]
    fn thirtytwo_iter() {
        let center = Coords { x: 0, y: 0 };
        let chunk_side_length = 32;
        let mut explorer = SpiralExplorer::new(center, chunk_side_length);

        assert_eq!(
            explorer.next(),
            Some(ChunkFootprint {
                bottomLeft: Coords {
                    x: 0,
                    y: chunk_side_length as i64,
                },
                sideLength: chunk_side_length as i64
            })
        );

        assert_eq!(
            explorer.next(),
            Some(ChunkFootprint {
                bottomLeft: Coords {
                    x: chunk_side_length as i64,
                    y: chunk_side_length as i64,
                },
                sideLength: chunk_side_length as i64
            })
        );

        assert_eq!(
            explorer.next(),
            Some(ChunkFootprint {
                bottomLeft: Coords {
                    x: chunk_side_length as i64,
                    y: 0,
                },
                sideLength: chunk_side_length as i64
            })
        );

        assert_eq!(
            explorer.next(),
            Some(ChunkFootprint {
                bottomLeft: Coords {
                    x: chunk_side_length as i64,
                    y: -(chunk_side_length as i64),
                },
                sideLength: chunk_side_length as i64
            })
        );

        assert_eq!(
            explorer.next(),
            Some(ChunkFootprint {
                bottomLeft: Coords {
                    x: 0,
                    y: -(chunk_side_length as i64),
                },
                sideLength: chunk_side_length as i64
            })
        );
    }
}
