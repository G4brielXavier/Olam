use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum Commands {

    /// To change Space's Metadata or Timeline's Metadata
    #[command(alias = "set")]
    Settings {

        /// To change the name
        #[arg(short, long)]
        title: Option<String>,

        /// To change the description
        #[arg(short, long)]
        desc: Option<String>,

        /// To change the Timeline's limit of years
        #[arg(short, long)]
        max: Option<u32>,

        /// To change the year name unit (as: "Ano", "Cycle", ...etc)
        #[arg(short, long)]
        yearname: Option<String>,

        /// To change the age name unit (as: "Era", "Generation", ...etc)
        #[arg(short, long)]
        agename: Option<String>

    },

    /// Create a new Space
    /// ## Example
    /// ```ps
    /// > olam evoke "MySpace"
    /// ```
    #[command(alias = "ev")]
    Evoke {

        /// Space's name
        name: String,
        
        /// Immerse directly in the Space evoked
        #[arg(short, long)]
        immerse: bool

    },

    /// Remove an existent space using your hash
    /// ## Example
    /// ```ps
    /// > olam efface a242296a
    /// ```
    #[command(alias = "ef")]
    Efface {

        /// Space's hash
        hash: String

    },

    /// Open and Enter in the specified:
    /// - Space (`OUTSPACE`)
    /// - Timeline (`INSPACE`)
    /// ## Example
    /// ```ps
    /// > olam immerse a242296a
    /// ```
    #[command(alias = "im")]
    Immerse {

        /// Space's hash
        hash: String

    },

    /// Leave from a Timeline (if is `INTIMELINE`) or a Space (if is `INSPACE`) <br>
    /// use: `olam ls` to see:
    /// - All Spaces (`OUTSPACE`)
    /// - All Timelines (`INSPACE`)
    #[command(alias = "em")]
    Emerge {

        #[arg(short, long)]
        out: bool 

    },

    /// List all Space (if is `OUTSPACE`) or Timelines (if is `INSPACE`)
    Ls,

    /// Show where the user is: `OUTSPACE`, `INSPACE` or `INTIMELINE`
    Stage,

    /// Create a new timeline to a space selected
    /// ## Example
    /// ```ps
    /// > olam add "my timeline"
    /// ```
    Add {

        /// Timeline's name
        name: String,
        
        /// Immerse directly in the timeline added
        #[arg(short, long)]
        immerse: bool

    },

    /// Delete a specified timeline
    /// ## Example
    /// ```ps
    /// > olam expun a242296a
    /// ```
    #[command(alias = "ex")]
    Expun {

        /// Timeline's hash
        hash: String

    },


    /// Add a `Year` to a `Timeline` that is immersed.
    /// ## Example
    /// ```ps
    /// olam year 10 "In this year..."
    /// ```
    /// 
    /// If you want add a `Year` inside an `Age` (in `< Events >`), use:
    /// ```ps
    /// olam year --age "AgeName" 10 "In this year in AgeName..."
    /// ```
    #[command(alias = "y")]
    Year {

        /// Age to add a Year inside
        #[arg(short, long)]
        age: Option<String>,

        /// Year value
        year: Option<u32>,

        /// Note to describe what happened in this year
        note: Option<String>

    },
    
    /// Add a `Age` to a `Timeline` that is immersed.
    /// ## Example
    /// ```ps
    /// olam age "AgeName" --disso 12
    /// ```
    #[command(alias = "a")]
    Age {

        /// Age's Name
        name: Option<String>,
        
        /// Represent in which `Year` the `Age` will be dissolved
        #[arg(short, long)]
        disso: Option<u32>,

        /// Add a note in `< Between >`
        #[arg(short, long)]
        between: Option<String>

    },


    /// Used to remove `Year`, `Age` or a note in `< Between >`
    Void {

        /// Year to remove
        #[arg(short, long)]
        year: Option<u32>,
        
        /// Age to remove
        #[arg(short, long)]
        age: Option<String>,

        /// Remove the last note of `< Between >` inside an `Age`
        #[arg(short, long)]
        pop: bool

    },

    /// Preview the `Timeline` that is immersed.
    #[command(alias = "tl")]
    Timeline

}