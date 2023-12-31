mod essential_oil {
    use crate::oil;
    use sea_orm::{
        ActiveModelBehavior, DeriveActiveEnum, DeriveEntityModel, DerivePrimaryKey, DeriveRelation,
        EntityTrait, EnumIter, PrimaryKeyTrait,
    };

    #[derive(EnumIter, DeriveActiveEnum, Clone, Debug, PartialEq)]
    #[sea_orm(rs_type = "u8", db_type = "Integer")]
    pub enum Note {
        #[sea_orm(num_value = 0)]
        Top,
        #[sea_orm(num_value = 1)]
        Middle,
        #[sea_orm(num_value = 2)]
        Base,
        #[sea_orm(num_value = 3)]
        TopAndMiddle,
        #[sea_orm(num_value = 4)]
        MiddleAndBase,
    }

    impl From<oil::Note> for Note {
        fn from(value: oil::Note) -> Self {
            match value {
                oil::Note::Simple(n) => match n {
                    oil::SimpleNote::Top => Self::Top,
                    oil::SimpleNote::Middle => Self::Middle,
                    oil::SimpleNote::Base => Self::Base,
                },
                oil::Note::TopAndMiddle => Self::TopAndMiddle,
                oil::Note::MiddleAndBase => Self::MiddleAndBase,
            }
        }
    }

    impl From<Note> for oil::Note {
        fn from(val: Note) -> oil::Note {
            match val {
                Note::Top => oil::Note::Simple(oil::SimpleNote::Top),
                Note::Middle => oil::Note::Simple(oil::SimpleNote::Middle),
                Note::Base => oil::Note::Simple(oil::SimpleNote::Base),
                Note::TopAndMiddle => oil::Note::TopAndMiddle,
                Note::MiddleAndBase => oil::Note::MiddleAndBase,
            }
        }
    }

    #[derive(EnumIter, DeriveActiveEnum, Clone, Debug, PartialEq)]
    #[sea_orm(rs_type = "u8", db_type = "Integer")]
    pub enum Strength {
        #[sea_orm(num_value = 0)]
        Week,
        #[sea_orm(num_value = 1)]
        Middle,
        #[sea_orm(num_value = 2)]
        Strong,
    }

    impl From<oil::Strength> for Strength {
        fn from(value: oil::Strength) -> Self {
            match value {
                oil::Strength::Week => Self::Week,
                oil::Strength::Middle => Self::Middle,
                oil::Strength::Strong => Self::Strong,
            }
        }
    }

    impl From<Strength> for oil::Strength {
        fn from(val: Strength) -> oil::Strength {
            match val {
                Strength::Week => oil::Strength::Week,
                Strength::Middle => oil::Strength::Middle,
                Strength::Strong => oil::Strength::Strong,
            }
        }
    }

    #[derive(Debug, Clone, DeriveEntityModel)]
    #[sea_orm(table_name = "essential_oil")]
    pub struct Model {
        #[sea_orm(primary_key)]
        pub id: uuid::Uuid,
        pub family: u8,
        pub name: String,
        pub remaining_amount: u8,
        pub note: Note,
        pub strength: Strength,
    }

    impl From<oil::EssentialOil> for Model {
        fn from(value: oil::EssentialOil) -> Self {
            Self {
                id: value.id,
                name: value.name,
                note: value.note.into(),
                family: value.family.bits(),
                remaining_amount: value.remaining_amount,
                strength: value.strength.into(),
            }
        }
    }

    #[derive(Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}
