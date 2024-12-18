use super::{DateTime, HangarPlatform, HangarVisibility};
use details::*;
use serde::Deserialize;
use std::rc::Rc;

mod details;
mod traits;

#[derive(Debug, Deserialize)]
pub struct HangarVersions {
  pub pagination: HangarVersionsPagination,
  pub result: Vec<HangarVersion>,
}

#[derive(Debug, Deserialize)]
pub struct HangarVersionsPagination {
  pub limit: u8,
  pub offset: u8,
  pub count: u16,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HangarVersion {
  pub created_at: DateTime,
  pub name: Rc<str>,
  pub visibility: HangarVisibility,
  pub description: Rc<str>,
  pub author: Rc<str>,
  #[serde(deserialize_with = "traits::deserialize_kv")]
  pub downloads: Vec<HangarVersionDownload>,
  #[serde(deserialize_with = "traits::deserialize_kv")]
  pub plugin_dependencies: Vec<HangarVersionPluginDependencies>,
  #[serde(deserialize_with = "traits::deserialize_kv")]
  pub platform_dependencies: Vec<HangarVersionPlatformDependencies>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HangarVersionDownload {
  pub platform: HangarPlatform,

  #[serde(flatten)]
  pub details: HPDownloadDetails,
}

impl traits::KeyValueType for HangarVersionDownload {
  type Key = HangarPlatform;
  type Value = HPDownloadDetails;

  fn init(key: Self::Key, value: Self::Value) -> Self {
    Self {
      platform: key,
      details: value,
    }
  }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HangarVersionPluginDependencies {
  pub name: Rc<str>,

  #[serde(flatten)]
  pub details: Rc<[HPPluginDependencyDetails]>,
}

impl traits::KeyValueType for HangarVersionPluginDependencies {
  type Key = Rc<str>;
  type Value = Rc<[HPPluginDependencyDetails]>;

  fn init(key: Self::Key, value: Self::Value) -> Self {
    Self {
      name: key,
      details: value,
    }
  }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HangarVersionPlatformDependencies {
  pub platform: HangarPlatform,
  pub version: Vec<Rc<str>>,
}

impl traits::KeyValueType for HangarVersionPlatformDependencies {
  type Key = HangarPlatform;
  type Value = Vec<Rc<str>>;

  fn init(key: Self::Key, value: Self::Value) -> Self {
    Self {
      platform: key,
      version: value,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use serde_json::from_str;

  #[test]
  fn one_version() {
    let raw = r#"
        {
            "createdAt": "2024-05-17T13:48:41.703391Z",
            "name": "4.2.1",
            "visibility": "public",
            "description": "Happy birthday Minecraft once again :)\n* The proxied maintenance status is now also shown for forced hosts (thanks to alec-jensen)\n* Offline player fetching improvements (thanks to EpicPlayerA10)\n  * Added the setting `fallback-to-offline-uuid` to use offline player uuids in maintenance whitelist commands (defaults to false)\n* Maintenance now skips the plugin remapping process on 1.20.5+ Paper servers\n* Small optimizations\n* Updated language files from [Crowdin](https://crowdin.com/translate/maintenance)\n  * Massive thank you to all the contributors! These include: vortix22, xdalul, CodeZhangBorui, jesusskye, Doc94,\n    Texider_, marvin2k0, leonard.bausenwein, pro.timeo.kerjean, meviper, troev5, Murka124, jhqwqmc, Pryzinho,\n    raysetratyboy, Kolja07, and rikunightcore",
            "stats": {
                "totalDownloads": 2608,
                "platformDownloads": {
                    "PAPER": 1764,
                    "WATERFALL": 102,
                    "VELOCITY": 742
                }
            },
            "author": "kennytv",
            "reviewState": "reviewed",
            "channel": {
                "createdAt": "2022-12-22T14:04:48.875876Z",
                "name": "Release",
                "description": null,
                "color": "00E1E1",
                "flags": [
                    "SENDS_NOTIFICATIONS",
                    "PINNED",
                    "FROZEN"
                ]
            },
            "pinnedStatus": "CHANNEL",
            "downloads": {
                "PAPER": {
                    "fileInfo": {
                        "name": "Maintenance-4.2.1.jar",
                        "sizeBytes": 1461873,
                        "sha256Hash": "fcc324e6df96b7a1570c4f65dfd144cfecfa03d759197af029d0954378656b87"
                    },
                    "externalUrl": null,
                    "downloadUrl": "https://hangarcdn.papermc.io/plugins/kennytv/Maintenance/versions/4.2.1/PAPER/Maintenance-4.2.1.jar"
                },
                "WATERFALL": {
                    "fileInfo": {
                        "name": "Maintenance-4.2.1.jar",
                        "sizeBytes": 1461873,
                        "sha256Hash": "fcc324e6df96b7a1570c4f65dfd144cfecfa03d759197af029d0954378656b87"
                    },
                    "externalUrl": null,
                    "downloadUrl": "https://hangarcdn.papermc.io/plugins/kennytv/Maintenance/versions/4.2.1/PAPER/Maintenance-4.2.1.jar"
                },
                "VELOCITY": {
                    "fileInfo": {
                        "name": "Maintenance-Velocity-4.2.1.jar",
                        "sizeBytes": 5375841,
                        "sha256Hash": "6a7c0ecc80ea30e7423ebf5754be7afb30e0b00d821423f6aff15d9f2287349e"
                    },
                    "externalUrl": null,
                    "downloadUrl": "https://hangarcdn.papermc.io/plugins/kennytv/Maintenance/versions/4.2.1/VELOCITY/Maintenance-Velocity-4.2.1.jar"
                }
            },
            "pluginDependencies": {
                "PAPER": [
                    {
                        "name": "ProtocolLib",
                        "required": false,
                        "externalUrl": null,
                        "platform": "PAPER"
                    }
                ]
            },
            "platformDependencies": {
                "PAPER": [
                    "1.10",
                    "1.10.1",
                    "1.10.2",
                    "1.11",
                    "1.11.1",
                    "1.11.2",
                    "1.12",
                    "1.12.1",
                    "1.12.2",
                    "1.13",
                    "1.13.1",
                    "1.13.2",
                    "1.14",
                    "1.14.1",
                    "1.14.2",
                    "1.14.3",
                    "1.14.4",
                    "1.15",
                    "1.15.1",
                    "1.15.2",
                    "1.16",
                    "1.16.1",
                    "1.16.2",
                    "1.16.3",
                    "1.16.4",
                    "1.16.5",
                    "1.17",
                    "1.17.1",
                    "1.18",
                    "1.18.1",
                    "1.18.2",
                    "1.19",
                    "1.19.1",
                    "1.19.2",
                    "1.19.3",
                    "1.19.4",
                    "1.20",
                    "1.20.1",
                    "1.20.2",
                    "1.20.3",
                    "1.20.4",
                    "1.20.5",
                    "1.20.6",
                    "1.21",
                    "1.8",
                    "1.9",
                    "1.9.1",
                    "1.9.2",
                    "1.9.3",
                    "1.9.4"
                ],
                "WATERFALL": [
                    "1.20"
                ],
                "VELOCITY": [
                    "3.0",
                    "3.1",
                    "3.2",
                    "3.3"
                ]
            },
            "platformDependenciesFormatted": {
                "PAPER": "1.8-1.21",
                "WATERFALL": "1.20",
                "VELOCITY": "3.0-3.3"
            }
        }
        "#;
    let version = from_str(raw);

    assert!(version.is_ok());
    let _version: HangarVersion = version.unwrap();
  }

  #[test]
  fn many_versions() {
    let raw = r#"
{
	"pagination": {
		"limit": 10,
		"offset": 0,
		"count": 4
	},
	"result": [
		{
			"createdAt": "2024-05-17T13:48:41.703391Z",
			"name": "4.2.1",
			"visibility": "public",
			"description": "Happy birthday Minecraft once again :)\n* The proxied maintenance status is now also shown for forced hosts (thanks to alec-jensen)\n* Offline player fetching improvements (thanks to EpicPlayerA10)\n  * Added the setting `fallback-to-offline-uuid` to use offline player uuids in maintenance whitelist commands (defaults to false)\n* Maintenance now skips the plugin remapping process on 1.20.5+ Paper servers\n* Small optimizations\n* Updated language files from [Crowdin](https://crowdin.com/translate/maintenance)\n  * Massive thank you to all the contributors! These include: vortix22, xdalul, CodeZhangBorui, jesusskye, Doc94,\n    Texider_, marvin2k0, leonard.bausenwein, pro.timeo.kerjean, meviper, troev5, Murka124, jhqwqmc, Pryzinho,\n    raysetratyboy, Kolja07, and rikunightcore",
			"stats": {
				"totalDownloads": 2608,
				"platformDownloads": {
					"PAPER": 1764,
					"WATERFALL": 102,
					"VELOCITY": 742
				}
			},
			"author": "kennytv",
			"reviewState": "reviewed",
			"channel": {
				"createdAt": "2022-12-22T14:04:48.875876Z",
				"name": "Release",
				"description": null,
				"color": "00E1E1",
				"flags": [
					"SENDS_NOTIFICATIONS",
					"PINNED",
					"FROZEN"
				]
			},
			"pinnedStatus": "CHANNEL",
			"downloads": {
				"PAPER": {
					"fileInfo": {
						"name": "Maintenance-4.2.1.jar",
						"sizeBytes": 1461873,
						"sha256Hash": "fcc324e6df96b7a1570c4f65dfd144cfecfa03d759197af029d0954378656b87"
					},
					"externalUrl": null,
					"downloadUrl": "https://hangarcdn.papermc.io/plugins/kennytv/Maintenance/versions/4.2.1/PAPER/Maintenance-4.2.1.jar"
				},
				"WATERFALL": {
					"fileInfo": {
						"name": "Maintenance-4.2.1.jar",
						"sizeBytes": 1461873,
						"sha256Hash": "fcc324e6df96b7a1570c4f65dfd144cfecfa03d759197af029d0954378656b87"
					},
					"externalUrl": null,
					"downloadUrl": "https://hangarcdn.papermc.io/plugins/kennytv/Maintenance/versions/4.2.1/PAPER/Maintenance-4.2.1.jar"
				},
				"VELOCITY": {
					"fileInfo": {
						"name": "Maintenance-Velocity-4.2.1.jar",
						"sizeBytes": 5375841,
						"sha256Hash": "6a7c0ecc80ea30e7423ebf5754be7afb30e0b00d821423f6aff15d9f2287349e"
					},
					"externalUrl": null,
					"downloadUrl": "https://hangarcdn.papermc.io/plugins/kennytv/Maintenance/versions/4.2.1/VELOCITY/Maintenance-Velocity-4.2.1.jar"
				}
			},
			"pluginDependencies": {
				"PAPER": [
					{
						"name": "ProtocolLib",
						"required": false,
						"externalUrl": null,
						"platform": "PAPER"
					}
				]
			},
			"platformDependencies": {
				"PAPER": [
					"1.10",
					"1.10.1",
					"1.10.2",
					"1.11",
					"1.11.1",
					"1.11.2",
					"1.12",
					"1.12.1",
					"1.12.2",
					"1.13",
					"1.13.1",
					"1.13.2",
					"1.14",
					"1.14.1",
					"1.14.2",
					"1.14.3",
					"1.14.4",
					"1.15",
					"1.15.1",
					"1.15.2",
					"1.16",
					"1.16.1",
					"1.16.2",
					"1.16.3",
					"1.16.4",
					"1.16.5",
					"1.17",
					"1.17.1",
					"1.18",
					"1.18.1",
					"1.18.2",
					"1.19",
					"1.19.1",
					"1.19.2",
					"1.19.3",
					"1.19.4",
					"1.20",
					"1.20.1",
					"1.20.2",
					"1.20.3",
					"1.20.4",
					"1.20.5",
					"1.20.6",
					"1.21",
					"1.8",
					"1.9",
					"1.9.1",
					"1.9.2",
					"1.9.3",
					"1.9.4"
				],
				"WATERFALL": [
					"1.20"
				],
				"VELOCITY": [
					"3.0",
					"3.1",
					"3.2",
					"3.3"
				]
			},
			"platformDependenciesFormatted": {
				"PAPER": "1.8-1.21",
				"WATERFALL": "1.20",
				"VELOCITY": "3.0-3.3"
			}
		},
		{
			"createdAt": "2023-09-17T09:49:43.197482Z",
			"name": "4.2.0",
			"visibility": "public",
			"description": "* Velocity/Bungee: Added `commands-on-single-maintenance-enable` and `commands-on-single-maintenance-disable` config options to define commands to be executed after maintenance has been toggled on a proxied server\n  * Commands inside the `all` list will be executed for any proxied server, but you can also define commands for specific servers by adding arrays with the server names as keys\n  * You can use the placeholder `%SERVER%` in commands to be replaced with the server name\n* Start-, end-, and schedule timers for proxied servers now also include the server in the feedback message\n* Updated language files from [Crowdin](https://crowdin.com/translate/maintenance) (including new language files for Danish, Japanese, Korean, and Hungarian)",
			"stats": {
				"totalDownloads": 4900,
				"platformDownloads": {
					"PAPER": 3096,
					"WATERFALL": 313,
					"VELOCITY": 1491
				}
			},
			"author": "kennytv",
			"reviewState": "reviewed",
			"channel": {
				"createdAt": "2022-12-22T14:04:48.875876Z",
				"name": "Release",
				"description": null,
				"color": "00E1E1",
				"flags": [
					"SENDS_NOTIFICATIONS",
					"PINNED",
					"FROZEN"
				]
			},
			"pinnedStatus": "CHANNEL",
			"downloads": {
				"PAPER": {
					"fileInfo": {
						"name": "Maintenance-4.2.0.jar",
						"sizeBytes": 1458539,
						"sha256Hash": "b85cf1f3528c60d29d98f2b0039129dd56a119979095ef8734fc81b38d6e3b9d"
					},
					"externalUrl": null,
					"downloadUrl": "https://hangarcdn.papermc.io/plugins/kennytv/Maintenance/versions/4.2.0/PAPER/Maintenance-4.2.0.jar"
				},
				"WATERFALL": {
					"fileInfo": {
						"name": "Maintenance-4.2.0.jar",
						"sizeBytes": 1458539,
						"sha256Hash": "b85cf1f3528c60d29d98f2b0039129dd56a119979095ef8734fc81b38d6e3b9d"
					},
					"externalUrl": null,
					"downloadUrl": "https://hangarcdn.papermc.io/plugins/kennytv/Maintenance/versions/4.2.0/PAPER/Maintenance-4.2.0.jar"
				},
				"VELOCITY": {
					"fileInfo": {
						"name": "Maintenance-Velocity-4.2.0.jar",
						"sizeBytes": 5371786,
						"sha256Hash": "f288f19e17bce4476a672efb3fb5c2ec3d4f1f6ad0f9a64b0c6e66ac44f94302"
					},
					"externalUrl": null,
					"downloadUrl": "https://hangarcdn.papermc.io/plugins/kennytv/Maintenance/versions/4.2.0/VELOCITY/Maintenance-Velocity-4.2.0.jar"
				}
			},
			"pluginDependencies": {
				"PAPER": [
					{
						"name": "ProtocolLib",
						"required": false,
						"externalUrl": null,
						"platform": "PAPER"
					}
				]
			},
			"platformDependencies": {
				"PAPER": [
					"1.10",
					"1.10.1",
					"1.10.2",
					"1.11",
					"1.11.1",
					"1.11.2",
					"1.12",
					"1.12.1",
					"1.12.2",
					"1.13",
					"1.13.1",
					"1.13.2",
					"1.14",
					"1.14.1",
					"1.14.2",
					"1.14.3",
					"1.14.4",
					"1.15",
					"1.15.1",
					"1.15.2",
					"1.16",
					"1.16.1",
					"1.16.2",
					"1.16.3",
					"1.16.4",
					"1.16.5",
					"1.17",
					"1.17.1",
					"1.18",
					"1.18.1",
					"1.18.2",
					"1.19",
					"1.19.1",
					"1.19.2",
					"1.19.3",
					"1.19.4",
					"1.20",
					"1.20.1",
					"1.20.2",
					"1.20.3",
					"1.20.4",
					"1.20.5",
					"1.20.6",
					"1.8",
					"1.9",
					"1.9.1",
					"1.9.2",
					"1.9.3",
					"1.9.4"
				],
				"WATERFALL": [
					"1.20"
				],
				"VELOCITY": [
					"3.0",
					"3.1",
					"3.2",
					"3.3"
				]
			},
			"platformDependenciesFormatted": {
				"PAPER": "1.8-1.20.6",
				"WATERFALL": "1.20",
				"VELOCITY": "3.0-3.3"
			}
		},
		{
			"createdAt": "2023-04-17T08:05:27.203321Z",
			"name": "4.1.0",
			"visibility": "public",
			"description": " Changed\n* Added config option `enable-playercounthovermessage` to be able to toggle the player count hover message\n* Removed platform specific command aliases (e.g. `maintenancevelocity`)\n* Translations\n  * Moved translation editing to [Crowdin](https://crowdin.com/Version/maintenance)\n  * Added Turkish translation file (thanks to Proomp)\n  * Added Swedish translation file (thanks to Sup33r)\n  * Added Ukrainian translation file (thanks to Mrlucke)\n  * Added Japanese translation file (thanks to yu-solt)\n* Added bStats metrics to Velocity and Sponge modules\n\n# Fixed\n* Fixed disabling the `enable-pingmessages` setting not working on Paper servers\n* Fixed variable replacement in messages with gradients\n* Fixed the message for cancelling proxied server timers not replacing the server argument\n* Setting `enable-pingmessages` to `false` no longer disables custom player and player hover messages",
			"stats": {
				"totalDownloads": 2040,
				"platformDownloads": {
					"PAPER": 1264,
					"WATERFALL": 138,
					"VELOCITY": 638
				}
			},
			"author": "kennytv",
			"reviewState": "reviewed",
			"channel": {
				"createdAt": "2022-12-22T14:04:48.875876Z",
				"name": "Release",
				"description": null,
				"color": "00E1E1",
				"flags": [
					"SENDS_NOTIFICATIONS",
					"PINNED",
					"FROZEN"
				]
			},
			"pinnedStatus": "CHANNEL",
			"downloads": {
				"PAPER": {
					"fileInfo": {
						"name": "Maintenance-4.1.0.jar",
						"sizeBytes": 1423182,
						"sha256Hash": "5295016054b1c874c9e478cc11458d4691d2799b2aa2bf18f04d3baccd3aa138"
					},
					"externalUrl": null,
					"downloadUrl": "https://hangarcdn.papermc.io/plugins/kennytv/Maintenance/versions/4.1.0/PAPER/Maintenance-4.1.0.jar"
				},
				"WATERFALL": {
					"fileInfo": {
						"name": "Maintenance-4.1.0.jar",
						"sizeBytes": 1423182,
						"sha256Hash": "5295016054b1c874c9e478cc11458d4691d2799b2aa2bf18f04d3baccd3aa138"
					},
					"externalUrl": null,
					"downloadUrl": "https://hangarcdn.papermc.io/plugins/kennytv/Maintenance/versions/4.1.0/PAPER/Maintenance-4.1.0.jar"
				},
				"VELOCITY": {
					"fileInfo": {
						"name": "Maintenance-4.1.0.jar",
						"sizeBytes": 5337654,
						"sha256Hash": "f0806817a4950838ce1a7580f8de28acd95f16f635c873ba2c9ebc73c89f3ba5"
					},
					"externalUrl": null,
					"downloadUrl": "https://hangarcdn.papermc.io/plugins/kennytv/Maintenance/versions/4.1.0/VELOCITY/Maintenance-4.1.0.jar"
				}
			},
			"pluginDependencies": {
				"PAPER": [
					{
						"name": "LuckPerms",
						"required": false,
						"externalUrl": "https://luckperms.net/",
						"platform": "PAPER"
					},
					{
						"name": "ProtocolLib",
						"required": false,
						"externalUrl": null,
						"platform": "PAPER"
					}
				],
				"WATERFALL": [
					{
						"name": "LuckPerms",
						"required": false,
						"externalUrl": "https://luckperms.net/",
						"platform": "WATERFALL"
					}
				],
				"VELOCITY": [
					{
						"name": "luckperms",
						"required": false,
						"externalUrl": "https://luckperms.net/",
						"platform": "VELOCITY"
					}
				]
			},
			"platformDependencies": {
				"PAPER": [
					"1.10",
					"1.10.1",
					"1.10.2",
					"1.11",
					"1.11.1",
					"1.11.2",
					"1.12",
					"1.12.1",
					"1.12.2",
					"1.13",
					"1.13.1",
					"1.13.2",
					"1.14",
					"1.14.1",
					"1.14.2",
					"1.14.3",
					"1.14.4",
					"1.15",
					"1.15.1",
					"1.15.2",
					"1.16",
					"1.16.1",
					"1.16.2",
					"1.16.3",
					"1.16.4",
					"1.16.5",
					"1.17",
					"1.17.1",
					"1.18",
					"1.18.1",
					"1.18.2",
					"1.19",
					"1.19.1",
					"1.19.2",
					"1.19.3",
					"1.19.4",
					"1.20",
					"1.8",
					"1.9",
					"1.9.1",
					"1.9.2",
					"1.9.3",
					"1.9.4"
				],
				"WATERFALL": [
					"1.20"
				],
				"VELOCITY": [
					"3.0",
					"3.1",
					"3.2"
				]
			},
			"platformDependenciesFormatted": {
				"PAPER": "1.8-1.20",
				"WATERFALL": "1.20",
				"VELOCITY": "3.0-3.2"
			}
		},
		{
			"createdAt": "2022-12-22T14:07:51.164194Z",
			"name": "4.0.1",
			"visibility": "public",
			"description": " Fixed\n* Fixed the config header sometimes breaking the config on upgrading/saving",
			"stats": {
				"totalDownloads": 112,
				"platformDownloads": {
					"PAPER": 53,
					"WATERFALL": 10,
					"VELOCITY": 49
				}
			},
			"author": "kennytv",
			"reviewState": "reviewed",
			"channel": {
				"createdAt": "2022-12-22T14:04:48.875876Z",
				"name": "Release",
				"description": null,
				"color": "00E1E1",
				"flags": [
					"SENDS_NOTIFICATIONS",
					"PINNED",
					"FROZEN"
				]
			},
			"pinnedStatus": "CHANNEL",
			"downloads": {
				"PAPER": {
					"fileInfo": {
						"name": "Maintenance.jar",
						"sizeBytes": 1328956,
						"sha256Hash": "4a569f01ac5251fcf17a34937a192fa68f2a4854be90baede3f2f6e132be9d4a"
					},
					"externalUrl": null,
					"downloadUrl": "https://hangarcdn.papermc.io/plugins/kennytv/Maintenance/versions/4.0.1/PAPER/Maintenance.jar"
				},
				"WATERFALL": {
					"fileInfo": {
						"name": "Maintenance.jar",
						"sizeBytes": 1328956,
						"sha256Hash": "4a569f01ac5251fcf17a34937a192fa68f2a4854be90baede3f2f6e132be9d4a"
					},
					"externalUrl": null,
					"downloadUrl": "https://hangarcdn.papermc.io/plugins/kennytv/Maintenance/versions/4.0.1/PAPER/Maintenance.jar"
				},
				"VELOCITY": {
					"fileInfo": {
						"name": "Maintenance.jar",
						"sizeBytes": 5230216,
						"sha256Hash": "1be446dfc9eed7469b1c6e96dc1ebdeee4ec744526e835916e161be7ddc590c9"
					},
					"externalUrl": null,
					"downloadUrl": "https://hangarcdn.papermc.io/plugins/kennytv/Maintenance/versions/4.0.1/VELOCITY/Maintenance.jar"
				}
			},
			"pluginDependencies": {
				"PAPER": [
					{
						"name": "LuckPerms",
						"required": false,
						"externalUrl": "https://luckperms.net/",
						"platform": "PAPER"
					},
					{
						"name": "ProtocolLib",
						"required": false,
						"externalUrl": "https://github.com/dmulloy2/ProtocolLib",
						"platform": "PAPER"
					},
					{
						"name": "ServerListPlus",
						"required": false,
						"externalUrl": "https://github.com/Minecrell/ServerListPlus",
						"platform": "PAPER"
					}
				],
				"WATERFALL": [
					{
						"name": "LuckPerms",
						"required": false,
						"externalUrl": "https://luckperms.net/",
						"platform": "WATERFALL"
					},
					{
						"name": "ServerListPlus",
						"required": false,
						"externalUrl": "https://github.com/Minecrell/ServerListPlus",
						"platform": "WATERFALL"
					}
				],
				"VELOCITY": [
					{
						"name": "luckperms",
						"required": false,
						"externalUrl": "https://luckperms.net/",
						"platform": "VELOCITY"
					},
					{
						"name": "serverlistplus",
						"required": false,
						"externalUrl": "https://github.com/Minecrell/ServerListPlus",
						"platform": "VELOCITY"
					}
				]
			},
			"platformDependencies": {
				"PAPER": [
					"1.10",
					"1.10.1",
					"1.10.2",
					"1.11",
					"1.11.1",
					"1.11.2",
					"1.12",
					"1.12.1",
					"1.12.2",
					"1.13",
					"1.13.1",
					"1.13.2",
					"1.14",
					"1.14.1",
					"1.14.2",
					"1.14.3",
					"1.14.4",
					"1.15",
					"1.15.1",
					"1.15.2",
					"1.16",
					"1.16.1",
					"1.16.2",
					"1.16.3",
					"1.16.4",
					"1.16.5",
					"1.17",
					"1.17.1",
					"1.18",
					"1.18.1",
					"1.18.2",
					"1.19",
					"1.19.1",
					"1.19.2",
					"1.19.3",
					"1.19.4",
					"1.8",
					"1.9",
					"1.9.1",
					"1.9.2",
					"1.9.3",
					"1.9.4"
				],
				"WATERFALL": [
					"1.19"
				],
				"VELOCITY": [
					"3.0",
					"3.1",
					"3.2"
				]
			},
			"platformDependenciesFormatted": {
				"PAPER": "1.8-1.19.4",
				"WATERFALL": "1.19",
				"VELOCITY": "3.0-3.2"
			}
		}
	]
}
        "#;

    let versions = from_str(raw);

    assert!(versions.is_ok());
    let _versions: HangarVersions = versions.unwrap();
  }
}
