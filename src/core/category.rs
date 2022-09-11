/* core/category.rs
 *
 * Copyright 2022 Jamie Murphy
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

use std::cmp::Ordering;

use appstream::prelude::*;
use appstream::Category;
use appstream::ComponentKind;

use crate::create_category;

use super::package::Package;

pub struct CatalogueCategories {
    pub accessories: Category,
    pub internet: Category,
    pub games: Category,
    pub develop: Category,
    pub create: Category,
    pub work: Category,
}

impl Default for CatalogueCategories {
    fn default() -> Self {
        Self {
            accessories: create_category!(
                "Accessories",
                "application-accessories",
                ("Utility", "Monitor", "System", "Accessibility")
            ),
            internet: create_category!(
                "Internet",
                "application-internet",
                (
                    "Chat",
                    "Email",
                    "InstantMessaging",
                    "IRCClient",
                    "VideoConference",
                    "Network",
                    "P2P"
                )
            ),
            games: create_category!(
                "Games",
                "application-games",
                (
                    "ActionGame",
                    "AdventureGame",
                    "ArcadeGame",
                    "Amusement",
                    "BlocksGame",
                    "BoardGame",
                    "CardGame",
                    "Game",
                    "KidsGame",
                    "LogicGame",
                    "RolePlaying",
                    "Shooter",
                    "Simulation",
                    "SportsGame",
                    "StrategyGame"
                )
            ),
            develop: create_category!(
                "Develop",
                "application-development",
                (
                    "Database",
                    "Debugger",
                    "Development",
                    "GUIDesigner",
                    "IDE",
                    "RevisionControl",
                    "TerminalEmulator",
                    "WebDevelopment"
                )
            ),
            create: create_category!(
                "Create",
                "applications-graphics",
                (
                    "2DGraphics",
                    "3DGraphics",
                    "Graphics",
                    "ImageProcessing",
                    "Photography",
                    "RasterGraphics",
                    "VectorGraphics",
                    "AudioVideoEditing",
                    "Midi",
                    "Mixer",
                    "Recorder",
                    "Sequencer",
                    "ArtificialIntelligence",
                    "Astronomy",
                    "Biology",
                    "Calculator",
                    "Chemistry",
                    "ComputerScience",
                    "DataVisualization",
                    "Electricity",
                    "Electronics",
                    "Engineering",
                    "Geology",
                    "Geoscience",
                    "Math",
                    "NumericalAnalysis",
                    "Physics",
                    "Robotics",
                    "Science",
                    "TV",
                    "Video",
                    "Audio",
                    "Music"
                )
            ),
            work: create_category!(
                "Work",
                "application-office",
                (
                    "Chat",
                    "ContactManagement",
                    "Email",
                    "InstantMessaging",
                    "IRCClient",
                    "Telephony",
                    "VideoConference",
                    "Economy",
                    "Finance",
                    "Network",
                    "P2P",
                    "Office",
                    "Presentation",
                    "Publishing",
                    "Spreadsheet",
                    "WordProcessor",
                    "Dictionary",
                    "Languages",
                    "Literature",
                    "OCR",
                    "TextEditor",
                    "TextTools",
                    "Translation"
                )
            ),
        }
    }
}

// Adds extra functions used for categories
pub trait CatalogueCategory {
    fn get_recently_updated_packages(&self, size: Option<usize>) -> Vec<Package>;
}

impl CatalogueCategory for Category {
    fn get_recently_updated_packages(&self, size: Option<usize>) -> Vec<Package> {
        let mut apps = Vec::new();
        // We don't want to modify the original list
        let mut packages = self.components().clone();

        // Sort by latest releases
        packages.sort_unstable_by(|p1, p2| {
            let p1_pkg = Package::new(p1.clone());
            let p2_pkg = Package::new(p2.clone());

            let p1_release = p1_pkg
                .get_latest_release()
                .map(|x| x.timestamp())
                .unwrap_or(0);
            let p2_release = p2_pkg
                .get_latest_release()
                .map(|x| x.timestamp())
                .unwrap_or(0);

            p2_release
                .partial_cmp(&p1_release)
                .unwrap_or(Ordering::Equal)
        });

        for package in packages.iter() {
            if apps.len() < size.unwrap_or(20) && package.kind() == ComponentKind::DesktopApp {
                apps.push(Package::new(package.clone()));
            }
        }

        apps
    }
}
