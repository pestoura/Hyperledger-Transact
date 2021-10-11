/*
 * Copyright 2021 Cargill Incorporated
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 * -----------------------------------------------------------------------------
 */

use crate::error::InternalError;
use crate::state::merkle::node::Node;
use crate::state::merkle::sql::backend::{self, Backend, Connection};

use super::operations::get_leaves::MerkleRadixGetLeavesOperation as _;
use super::operations::get_or_create_tree::MerkleRadixGetOrCreateTreeOperation as _;
use super::operations::get_path::MerkleRadixGetPathOperation as _;
use super::operations::get_tree_by_name::MerkleRadixGetTreeByNameOperation as _;
use super::operations::has_root::MerkleRadixHasRootOperation as _;
use super::operations::list_leaves::MerkleRadixListLeavesOperation as _;
use super::operations::prune_entries::MerkleRadixPruneEntriesOperation as _;
use super::operations::write_changes::MerkleRadixWriteChangesOperation as _;
use super::operations::MerkleRadixOperations;
use super::{MerkleRadixStore, SqlMerkleRadixStore, TreeUpdate};

impl<'b> MerkleRadixStore for SqlMerkleRadixStore<'b, backend::PostgresBackend> {
    fn get_or_create_tree(
        &self,
        tree_name: &str,
        initial_state_root_hash: &str,
    ) -> Result<i64, InternalError> {
        let conn = self.backend.connection()?;
        let operations = MerkleRadixOperations::new(conn.as_inner());
        operations.get_or_create_tree(tree_name, initial_state_root_hash)
    }

    fn get_tree_id_by_name(&self, tree_name: &str) -> Result<Option<i64>, InternalError> {
        let conn = self.backend.connection()?;
        let operations = MerkleRadixOperations::new(conn.as_inner());
        operations.get_tree_id_by_name(tree_name)
    }
    fn has_root(&self, tree_id: i64, state_root_hash: &str) -> Result<bool, InternalError> {
        let conn = self.backend.connection()?;

        let operations = MerkleRadixOperations::new(conn.as_inner());
        operations.has_root(tree_id, state_root_hash)
    }

    fn get_path(
        &self,
        tree_id: i64,
        state_root_hash: &str,
        address: &str,
    ) -> Result<Vec<(String, Node)>, InternalError> {
        let conn = self.backend.connection()?;

        let operations = MerkleRadixOperations::new(conn.as_inner());
        operations.get_path(tree_id, state_root_hash, address)
    }

    fn get_entries(
        &self,
        tree_id: i64,
        state_root_hash: &str,
        keys: Vec<&str>,
    ) -> Result<Vec<(String, Vec<u8>)>, InternalError> {
        let conn = self.backend.connection()?;

        let operations = MerkleRadixOperations::new(conn.as_inner());
        operations.get_leaves(tree_id, state_root_hash, keys)
    }

    fn list_entries(
        &self,
        tree_id: i64,
        state_root_hash: &str,
        prefix: Option<&str>,
    ) -> Result<Vec<(String, Vec<u8>)>, InternalError> {
        let conn = self.backend.connection()?;

        let operations = MerkleRadixOperations::new(conn.as_inner());
        operations.list_leaves(tree_id, state_root_hash, prefix)
    }

    fn write_changes(
        &self,
        tree_id: i64,
        state_root_hash: &str,
        parent_state_root_hash: &str,
        tree_update: TreeUpdate,
    ) -> Result<(), InternalError> {
        let conn = self.backend.connection()?;

        let operations = MerkleRadixOperations::new(conn.as_inner());

        operations.write_changes(
            tree_id,
            state_root_hash,
            parent_state_root_hash,
            &tree_update,
        )?;
        Ok(())
    }

    fn prune(&self, tree_id: i64, state_root: &str) -> Result<Vec<String>, InternalError> {
        let conn = self.backend.connection()?;
        let operations = MerkleRadixOperations::new(conn.as_inner());
        operations.prune_entries(tree_id, state_root)
    }
}