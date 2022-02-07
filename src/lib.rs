use bevy::prelude::*;

#[derive(Bundle)]
pub struct MarkedBundle<B, M> 
where
    B: Bundle,
    M: Bundle,
{
    #[bundle]
    bundle: B,
    #[bundle]
    marker: M,
}

impl <B, M> MarkedBundle<B, M> 
where 
    B: Bundle,
    M: Bundle + Default,
{ 
    pub fn new(bundle: B) -> Self {
        Self {
            bundle,
            marker: Default::default(),
        }
    }
}

#[macro_export]
macro_rules! marked_commands {
    ($M:ty) => {
        use bevy::ecs::system::EntityCommands;
        use bevy::ecs::system::SpawnBatch;
        #[derive(Bundle)]
        pub struct MarkedBundle<B, M> 
        where
            B: Bundle,
            M: Bundle,
        {
            #[bundle]
            bundle: B,
            #[bundle]
            marker: M,
        }
        
        impl <B, M> MarkedBundle<B, M> 
        where 
            B: Bundle,
            M: Bundle + Default,
        { 
            pub fn new(bundle: B) -> Self {
                Self {
                    bundle,
                    marker: Default::default(),
                }
            }
        }

        pub trait MarkEntityCommandsExt {
            fn mark(&mut self) -> &mut Self;
        }

        impl MarkEntityCommandsExt for EntityCommands<'_, '_, '_> {
            fn mark(&mut self) -> &mut Self {
                self.insert_bundle(<$M>::default());
                self
            }
        }

        pub trait MarkedCommandsExt<'w, 's> {            
            fn marked<'a>(&'a mut self) -> EntityCommands<'w, 's, 'a>;
            fn marked_bundle<'a, B: Bundle>(&'a mut self, bundle: B) -> EntityCommands<'w, 's, 'a>;
        }

        pub trait MarkedOrExt<'w, 's> {
            fn get_or_marked<'a>(&'a mut self, entity: Entity) -> EntityCommands<'w, 's, 'a>;
            fn mark_and_forget<'a>(&'a mut self, bundle: impl Bundle);
        }
        
        impl <'w, 's> MarkedOrExt<'w, 's> for Commands<'w, 's> {
            fn get_or_marked<'a>(&'a mut self, entity: Entity) -> EntityCommands<'w, 's, 'a> {
                let mut e = self.get_or_spawn(entity);
                e.insert_bundle(<$M>::default());
                e
            }

            fn mark_and_forget<'a>(&'a mut self, bundle: impl Bundle) {
                self.spawn_and_forget(MarkedBundle::<_, $M>::new(bundle));
            }
        }

        pub trait MarkedInsertBundleExt {
            fn marked_bundle<B: Bundle>(self, bundle: B) -> Self;
        }

        impl MarkedInsertBundleExt for EntityCommands<'_, '_, '_> {
            fn marked_bundle<B: Bundle>(mut self, bundle: B) -> Self {
                self
                .insert_bundle(MarkedBundle::<_, $M>::new(bundle));
                self
            }
        }

        pub trait MarkedCommandsBatchExt {
            fn marked_batch<I>(&mut self, bundles_iter: I) 
            where 
                I: IntoIterator + Send + Sync + 'static,
                I::Item: Bundle,
                I::IntoIter: Send + Sync;
        }

        impl <'w, 's> MarkedCommandsExt<'w, 's>  for Commands<'w, 's> {
            fn marked(&mut self) -> EntityCommands<'w, 's, '_> {
                let mut entity_commands = self.spawn();
                entity_commands.insert_bundle(<$M>::default());
                entity_commands
            }
        
            fn marked_bundle<B>(&mut self, bundle: B) -> EntityCommands<'w, 's, '_>
            where 
                B: Bundle
            {
                let mut entity_commands = self.spawn_bundle(MarkedBundle::<_, $M>::new(bundle));
                entity_commands
            }
        }

        impl <'w, 's> MarkedCommandsBatchExt for Commands<'w, 's> {
            fn marked_batch<I>(&mut self, bundles_iter: I) 
            where 
                I: IntoIterator + Send + Sync + 'static,
                I::Item: Bundle,                
                I::IntoIter: Send + Sync,
            {
                let marked_bundles = 
                    bundles_iter.into_iter()
                    .map(|bundle| MarkedBundle::<_, $M>::new(bundle));
                self.spawn_batch(marked_bundles);
            }
        }
        
        impl <'w, 's, 'a> MarkedCommandsExt<'w, 's>  for ChildBuilder<'w, 's, 'a> {
            fn marked(&mut self) -> EntityCommands<'w, 's, '_> {
                let mut entity_commands = self.spawn();
                entity_commands.insert_bundle(<$M>::default());
                entity_commands
            }
        
            fn marked_bundle<B>(&mut self, bundle: B) -> EntityCommands<'w, 's, '_>
            where 
                B: Bundle
            {
                let mut entity_commands = self.spawn_bundle(MarkedBundle::<_, $M>::new(bundle));
                entity_commands
            }
        }
    }
}