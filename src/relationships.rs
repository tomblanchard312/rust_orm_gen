use std::any::Any;

pub enum RelationType {
    OneToOne,
    OneToMany,
    ManyToMany,
}
pub struct Relationship {
    pub relation_type: RelationType,
    pub foreign_key: String,
    pub related_table: String,
}
impl Relationship {
    pub fn new(relation_type: RelationType, foreign_key: &str, related_table: &str) -> Box<dyn Any> {
        Box::new(Self {
            relation_type,
            foreign_key: foreign_key.to_string(),
            related_table: related_table.to_string(),
        })
    }
}

pub trait HasRelationships {
    fn relationships() -> Vec<Box<dyn Any>>;
}

// Example implementation
pub struct User;

impl HasRelationships for User {
    fn relationships() -> Vec<Box<dyn Any>> {
        vec![
            Relationship::new(RelationType::OneToMany, "user_id", "posts"),
            Relationship::new(RelationType::OneToOne, "user_id", "profile"),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_relationship_creation() {
        let rel = Relationship::new(RelationType::OneToMany, "user_id", "posts");
        let rel_any = rel.downcast_ref::<Relationship>().unwrap();
        assert!(matches!(rel_any.relation_type, RelationType::OneToMany));
        assert_eq!(rel_any.foreign_key, "user_id");
        assert_eq!(rel_any.related_table, "posts");
    }

    #[test]
    fn test_user_relationships() {
        let relationships = User::relationships();
        assert_eq!(relationships.len(), 2);

        let posts_rel = relationships[0].downcast_ref::<Relationship>().unwrap();
        assert!(matches!(posts_rel.relation_type, RelationType::OneToMany));
        assert_eq!(posts_rel.foreign_key, "user_id");
        assert_eq!(posts_rel.related_table, "posts");

        let profile_rel = relationships[1].downcast_ref::<Relationship>().unwrap();
        assert!(matches!(profile_rel.relation_type, RelationType::OneToOne));
        assert_eq!(profile_rel.foreign_key, "user_id");
        assert_eq!(profile_rel.related_table, "profile");
    }
}