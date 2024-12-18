use super::{DateTime, HangarTags, HangarVisibility};
use serde::Deserialize;
use std::{fmt::Debug, rc::Rc};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HangarProjects {
  pub pagination: HangarProjectsPagination,
  pub result: Rc<[HangarProject]>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HangarProjectsPagination {
  pub limit: u8,
  pub offset: u8,
  pub count: u16,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HangarProject {
  pub created_at: DateTime,
  pub name: Rc<str>,
  pub namespace: HangarProjectNamespace,
  pub last_updated: DateTime,
  pub avatar_url: Rc<str>,
  pub description: Rc<str>,
  pub category: HangarProjectCategory,
  pub visibility: HangarVisibility,
  pub settings: HangarProjectSettings,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HangarProjectSettings {
  pub links: Option<Rc<[HangarProjectLinks]>>,
  pub tags: HangarTags,
  pub license: HangarProjectLicense,
  pub keywords: Vec<Rc<str>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HangarProjectNamespace {
  pub owner: Rc<str>,
  pub slug: Rc<str>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HangarProjectLinks {
  #[serde(deserialize_with = "deserialize_links")]
  pub links: Rc<[HangarProjectLink]>,
}

#[derive(Debug, Deserialize)]
pub struct HangarProjectLink {
  pub id: u8,
  pub name: Rc<str>,
  #[serde(deserialize_with = "deserialize_null_default")]
  pub url: Rc<str>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HangarProjectCategory {
  AdminTools,
  Chat,
  DevTools,
  Economy,
  Gameplay,
  Games,
  Protection,
  RolePlaying,
  WorldManagement,
  Misc,
  Undefined,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HangarProjectLicense {
  pub name: Option<Rc<str>>,
  pub url: Option<Rc<str>>,

  #[serde(rename = "type")]
  pub license_type: Rc<str>,
}

fn deserialize_links<'de, D>(deserializer: D) -> Result<Rc<[HangarProjectLink]>, D::Error>
where
  D: serde::Deserializer<'de>,
{
  struct LinksVisitor;

  impl<'de> serde::de::Visitor<'de> for LinksVisitor {
    type Value = Rc<[HangarProjectLink]>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
      formatter.write_str("a list of HangarProjectLinks")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
      A: serde::de::SeqAccess<'de>,
    {
      let mut links = Vec::new();

      while let Some(link) = seq.next_element::<HangarProjectLink>()? {
        if !link.url.is_empty() {
          links.push(link);
        }
      }

      Ok(Rc::from_iter(links.into_boxed_slice()))
    }
  }

  deserializer.deserialize_seq(LinksVisitor)
}

fn deserialize_null_default<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
  T: Default + Deserialize<'de>,
  D: serde::Deserializer<'de>,
{
  let opt = Option::deserialize(deserializer)?;
  Ok(opt.unwrap_or_default())
}

#[cfg(test)]
mod tests {
  use super::*;
  use serde_json::from_str;

  #[test]
  fn one_project() {
    let raw = r#"
{
	"createdAt": "2022-12-22T14:04:48.773082Z",
	"name": "Maintenance",
	"namespace": {
		"owner": "kennytv",
		"slug": "Maintenance"
	},
	"stats": {
		"views": 49672,
		"downloads": 9660,
		"recentViews": 5414,
		"recentDownloads": 1395,
		"stars": 106,
		"watchers": 61
	},
	"category": "admin_tools",
	"lastUpdated": "2024-05-17T13:48:41.703391Z",
	"visibility": "public",
	"avatarUrl": "https://hangarcdn.papermc.io/avatars/project/1.webp?v=1",
	"description": "The most customizable maintenance plugin for your Minecraft server you can find.",
	"userActions": {
		"starred": false,
		"watching": false,
		"flagged": false
	},
	"settings": {
		"links": [
			{
				"id": 0,
				"type": "top",
				"title": "top",
				"links": [
					{
						"id": 5,
						"name": "Donate",
						"url": "https://github.com/sponsors/kennytv"
					},
					{
						"id": 1,
						"name": "Issues",
						"url": "https://github.com/kennytv/Maintenance/issues"
					},
					{
						"id": 2,
						"name": "Source",
						"url": "https://github.com/kennytv/Maintenance"
					},
					{
						"id": 3,
						"name": "Support",
						"url": "https://discord.gg/vGCUzHq"
					},
					{
						"id": 4,
						"name": "Wiki",
						"url": "https://github.com/kennytv/Maintenance/wiki"
					}
				]
			}
		],
		"tags": [],
		"license": {
			"name": "GPL",
			"url": "https://github.com/kennytv/Maintenance/blob/main/LICENSE.txt",
			"type": "GPL"
		},
		"keywords": [
			"maintenance",
			"maintenancemode"
		],
		"sponsors": "",
		"donation": {
			"enable": false,
			"subject": ""
		}
	}
}"#;
    let project = from_str(raw);
    assert!(project.is_ok());

    let _project: HangarProject = project.unwrap();
  }

  #[test]
  fn many_projects() {
    let raw = r#"
{
  "pagination": {
    "limit": 25,
    "offset": 0,
    "count": 1237
  },
  "result": [
    {
      "createdAt": "2022-12-22T14:04:48.773082Z",
      "name": "Maintenance",
      "namespace": {
        "owner": "kennytv",
        "slug": "Maintenance"
      },
      "stats": {
        "views": 49825,
        "downloads": 9703,
        "recentViews": 5399,
        "recentDownloads": 1397,
        "stars": 106,
        "watchers": 61
      },
      "category": "admin_tools",
      "lastUpdated": "2024-05-17T13:48:41.703391Z",
      "visibility": "public",
      "avatarUrl": "https://hangarcdn.papermc.io/avatars/project/1.webp?v=1",
      "description": "The most customizable maintenance plugin for your Minecraft server you can find.",
      "userActions": {
        "starred": false,
        "watching": false,
        "flagged": false
      },
      "settings": {
        "links": [
          {
            "id": 0,
            "type": "top",
            "title": "top",
            "links": [
              {
                "id": 5,
                "name": "Donate",
                "url": "https://github.com/sponsors/kennytv"
              },
              {
                "id": 1,
                "name": "Issues",
                "url": "https://github.com/kennytv/Maintenance/issues"
              },
              {
                "id": 2,
                "name": "Source",
                "url": "https://github.com/kennytv/Maintenance"
              },
              {
                "id": 3,
                "name": "Support",
                "url": "https://discord.gg/vGCUzHq"
              },
              {
                "id": 4,
                "name": "Wiki",
                "url": "https://github.com/kennytv/Maintenance/wiki"
              }
            ]
          }
        ],
        "tags": [],
        "license": {
          "name": "GPL",
          "url": "https://github.com/kennytv/Maintenance/blob/main/LICENSE.txt",
          "type": "GPL"
        },
        "keywords": [
          "maintenance",
          "maintenancemode"
        ],
        "sponsors": "",
        "donation": {
          "enable": false,
          "subject": ""
        }
      }
    },
    {
      "createdAt": "2022-12-22T14:15:52.547268Z",
      "name": "serena",
      "namespace": {
        "owner": "lynxplay",
        "slug": "serena"
      },
      "stats": {
        "views": 1463,
        "downloads": 97,
        "recentViews": 102,
        "recentDownloads": 6,
        "stars": 2,
        "watchers": 2
      },
      "category": "gameplay",
      "lastUpdated": "2022-12-22T14:18:50.488683Z",
      "visibility": "public",
      "avatarUrl": "https://hangarcdn.papermc.io/avatars/project/2.webp?v=1",
      "description": "Pick up your friends and carry them on your head!",
      "userActions": {
        "starred": false,
        "watching": false,
        "flagged": false
      },
      "settings": {
        "links": [
          {
            "id": 0,
            "type": "top",
            "title": "top",
            "links": [
              {
                "id": 0,
                "name": "Homepage",
                "url": "https://github.com/lynxplay/serena"
              },
              {
                "id": 1,
                "name": "Issues",
                "url": "https://github.com/lynxplay/serena/issues"
              },
              {
                "id": 2,
                "name": "Source",
                "url": "https://github.com/lynxplay/serena"
              },
              {
                "id": 3,
                "name": "Support",
                "url": null
              },
              {
                "id": 4,
                "name": "Wiki",
                "url": null
              }
            ]
          }
        ],
        "tags": [],
        "license": {
          "name": null,
          "url": null,
          "type": "MIT"
        },
        "keywords": [
          "hoggyback",
          "head",
          "pickup"
        ],
        "sponsors": "",
        "donation": {
          "enable": false,
          "subject": ""
        }
      }
    },
    {
      "createdAt": "2022-12-22T18:22:07.211685Z",
      "name": "UnifiedMetrics",
      "namespace": {
        "owner": "cubxity",
        "slug": "UnifiedMetrics"
      },
      "stats": {
        "views": 6076,
        "downloads": 538,
        "recentViews": 510,
        "recentDownloads": 46,
        "stars": 24,
        "watchers": 7
      },
      "category": "admin_tools",
      "lastUpdated": "2023-04-23T19:34:08.702899Z",
      "visibility": "public",
      "avatarUrl": "https://hangarcdn.papermc.io/avatars/project/3.webp?v=1",
      "description": "Fully-featured free and open-source metrics collection plugin for Minecraft servers",
      "userActions": {
        "starred": false,
        "watching": false,
        "flagged": false
      },
      "settings": {
        "links": [
          {
            "id": 0,
            "type": "top",
            "title": "top",
            "links": [
              {
                "id": 0,
                "name": "Homepage",
                "url": "https://github.com/cubxity/unifiedmetrics"
              },
              {
                "id": 1,
                "name": "Issues",
                "url": "https://github.com/cubxity/unifiedmetrics/issues"
              },
              {
                "id": 2,
                "name": "Source",
                "url": "https://github.com/cubxity/unifiedmetrics"
              },
              {
                "id": 3,
                "name": "Support",
                "url": "https://discord.gg/kDDhqJmPpA"
              },
              {
                "id": 4,
                "name": "Wiki",
                "url": "https://docs.cubxity.dev/docs/unifiedmetrics/intro"
              }
            ]
          }
        ],
        "tags": [],
        "license": {
          "name": null,
          "url": "https://github.com/Cubxity/UnifiedMetrics/blob/dev/0.3.x/COPYING.LESSER",
          "type": "LGPL"
        },
        "keywords": [
          "metrics",
          "prometheus",
          "grafana",
          "monitoring"
        ],
        "sponsors": "[Bloom Host](https://billing.bloom.host/aff.php?aff=9) has kindly provided UnifiedMetrics with development servers.\nBloom has server splitting built-in, which makes it extremely easy to build your monitoring stack. Get high performance\nservers at Bloom by using [this link](https://billing.bloom.host/aff.php?aff=9).",
        "donation": {
          "enable": false,
          "subject": ""
        }
      }
    },
    {
      "createdAt": "2022-12-22T20:52:18.999416Z",
      "name": "Chunky",
      "namespace": {
        "owner": "pop4959",
        "slug": "Chunky"
      },
      "stats": {
        "views": 52018,
        "downloads": 17171,
        "recentViews": 6350,
        "recentDownloads": 2332,
        "stars": 124,
        "watchers": 81
      },
      "category": "world_management",
      "lastUpdated": "2024-05-06T06:43:49.086114Z",
      "visibility": "public",
      "avatarUrl": "https://hangarcdn.papermc.io/avatars/project/4.webp?v=1",
      "description": "Pre-generates chunks, quickly and efficiently",
      "userActions": {
        "starred": false,
        "watching": false,
        "flagged": false
      },
      "settings": {
        "links": [
          {
            "id": 0,
            "type": "top",
            "title": "top",
            "links": [
              {
                "id": 0,
                "name": "Homepage",
                "url": "https://github.com/pop4959/Chunky"
              },
              {
                "id": 1,
                "name": "Issues",
                "url": "https://github.com/pop4959/Chunky/issues"
              },
              {
                "id": 2,
                "name": "Source",
                "url": "https://github.com/pop4959/Chunky"
              },
              {
                "id": 3,
                "name": "Support",
                "url": "https://discord.gg/ZwVJukcNQG"
              },
              {
                "id": 4,
                "name": "Wiki",
                "url": "https://github.com/pop4959/Chunky/wiki"
              }
            ]
          }
        ],
        "tags": [
          "SUPPORTS_FOLIA"
        ],
        "license": {
          "name": "GPL",
          "url": "https://github.com/pop4959/Chunky/blob/master/LICENSE",
          "type": "GPL"
        },
        "keywords": [],
        "sponsors": "",
        "donation": {
          "enable": false,
          "subject": ""
        }
      }
    },
    {
      "createdAt": "2022-12-22T21:33:24.066351Z",
      "name": "WorldEditSUI",
      "namespace": {
        "owner": "kennytv",
        "slug": "WorldEditSUI"
      },
      "stats": {
        "views": 11003,
        "downloads": 2949,
        "recentViews": 1139,
        "recentDownloads": 303,
        "stars": 33,
        "watchers": 21
      },
      "category": "admin_tools",
      "lastUpdated": "2024-07-19T12:50:57.406109Z",
      "visibility": "public",
      "avatarUrl": "https://hangarcdn.papermc.io/avatars/project/5.webp?v=1",
      "description": "Visualize your WorldEdit selection with particles!",
      "userActions": {
        "starred": false,
        "watching": false,
        "flagged": false
      },
      "settings": {
        "links": [
          {
            "id": 0,
            "type": "top",
            "title": "top",
            "links": [
              {
                "id": 4,
                "name": "Donate",
                "url": "https://github.com/sponsors/kennytv"
              },
              {
                "id": 1,
                "name": "Issues",
                "url": "https://github.com/kennytv/WorldEditSUI/issues"
              },
              {
                "id": 2,
                "name": "Source",
                "url": "https://github.com/kennytv/WorldEditSUI"
              },
              {
                "id": 3,
                "name": "Support",
                "url": "https://discord.gg/vGCUzHq"
              }
            ]
          }
        ],
        "tags": [
          "ADDON"
        ],
        "license": {
          "name": "Unspecified",
          "url": null,
          "type": "Unspecified"
        },
        "keywords": [
          "worldedit",
          "worldeditsui",
          "selection",
          "visualizer"
        ],
        "sponsors": "",
        "donation": {
          "enable": false,
          "subject": ""
        }
      }
    },
    {
      "createdAt": "2022-12-23T06:58:21.584949Z",
      "name": "WorldEdit",
      "namespace": {
        "owner": "EngineHub",
        "slug": "WorldEdit"
      },
      "stats": {
        "views": 4537,
        "downloads": 2478,
        "recentViews": 2408,
        "recentDownloads": 1320,
        "stars": 6,
        "watchers": 5
      },
      "category": "admin_tools",
      "lastUpdated": "2024-07-06T11:15:06.021765Z",
      "visibility": "public",
      "avatarUrl": "https://hangarcdn.papermc.io/avatars/project/6.webp?v=1",
      "description": "A Minecraft Map Editor... that runs in-game! With selections, schematics, copy and paste, brushes, and scripting",
      "userActions": {
        "starred": false,
        "watching": false,
        "flagged": false
      },
      "settings": {
        "links": [
          {
            "id": 0,
            "type": "top",
            "title": "top",
            "links": [
              {
                "id": 0,
                "name": "Homepage",
                "url": "https://enginehub.org/worldedit"
              },
              {
                "id": 1,
                "name": "Issues",
                "url": "https://github.com/EngineHub/WorldEdit/issues"
              },
              {
                "id": 2,
                "name": "Source",
                "url": "https://github.com/EngineHub/WorldEdit"
              },
              {
                "id": 3,
                "name": "Support",
                "url": "https://discord.gg/enginehub"
              },
              {
                "id": 4,
                "name": "Wiki",
                "url": "https://worldedit.enginehub.org/en/latest/"
              }
            ]
          }
        ],
        "tags": [],
        "license": {
          "name": "Unspecified",
          "url": "https://github.com/EngineHub/WorldEdit/blob/master/LICENSE.txt",
          "type": "GPL"
        },
        "keywords": [
          "worldedit",
          "world",
          "editor",
          "brushes",
          "sculpting"
        ],
        "sponsors": "<center>\n\n[🎉 Support us on GitHub Sponsors!](https://github.com/sponsors/EngineHub)\n\n[Need a host? We recommend Apex Hosting](https://billing.apexminecrafthosting.com/aff.php?aff=3108)\n\n[![Apex Hosting](https://enginehub.org/images/apex.svg==91x334)](https://billing.apexminecrafthosting.com/aff.php?aff=3108)\n  \n </center>",
        "donation": {
          "enable": false,
          "subject": ""
        }
      }
    },
    {
      "createdAt": "2022-12-23T19:37:57.240075Z",
      "name": "NotQuests",
      "namespace": {
        "owner": "Alessio",
        "slug": "NotQuests"
      },
      "stats": {
        "views": 7919,
        "downloads": 684,
        "recentViews": 518,
        "recentDownloads": 54,
        "stars": 15,
        "watchers": 10
      },
      "category": "admin_tools",
      "lastUpdated": "2024-05-04T22:40:41.083432Z",
      "visibility": "public",
      "avatarUrl": "https://hangarcdn.papermc.io/avatars/project/7.webp?v=3",
      "description": "Best, Paper-Native Quest Plugin ✅ Better than \"Quests\" ⭐ Tasks ⭐ Multi-Path Objectives & Sub-Objectives ⭐ Conversations",
      "userActions": {
        "starred": false,
        "watching": false,
        "flagged": false
      },
      "settings": {
        "links": [
          {
            "id": 0,
            "type": "top",
            "title": "top",
            "links": [
              {
                "id": 0,
                "name": "Homepage",
                "url": "https://www.notquests.com/"
              },
              {
                "id": 1,
                "name": "Issues",
                "url": "https://github.com/AlessioGr/NotQuests/issues"
              },
              {
                "id": 2,
                "name": "Source",
                "url": "https://github.com/AlessioGr/NotQuests"
              },
              {
                "id": 3,
                "name": "Support",
                "url": "https://discord.gg/7br638S5Ex"
              },
              {
                "id": 4,
                "name": "Wiki",
                "url": "https://www.notquests.com/docs/tutorials/getting-started"
              }
            ]
          }
        ],
        "tags": [],
        "license": {
          "name": "GPL",
          "url": "https://github.com/AlessioGr/NotQuests/blob/main/LICENSE",
          "type": "GPL"
        },
        "keywords": [
          "quest",
          "quests",
          "questing",
          "rpg",
          "conversation"
        ],
        "sponsors": "",
        "donation": {
          "enable": false,
          "subject": ""
        }
      }
    },
    {
      "createdAt": "2022-12-23T20:17:16.903766Z",
      "name": "GravityControl",
      "namespace": {
        "owner": "sulu",
        "slug": "GravityControl"
      },
      "stats": {
        "views": 1153,
        "downloads": 68,
        "recentViews": 96,
        "recentDownloads": 6,
        "stars": 0,
        "watchers": 1
      },
      "category": "admin_tools",
      "lastUpdated": "2022-12-23T20:22:08.074997Z",
      "visibility": "public",
      "avatarUrl": "https://hangarcdn.papermc.io/avatars/project/8.webp?v=1",
      "description": "Liberate your server from the anti-dupe bourgeoisie! Enable sand duping on your Paper server.",
      "userActions": {
        "starred": false,
        "watching": false,
        "flagged": false
      },
      "settings": {
        "links": [
          {
            "id": 0,
            "type": "top",
            "title": "top",
            "links": [
              {
                "id": 0,
                "name": "Homepage",
                "url": null
              },
              {
                "id": 1,
                "name": "Issues",
                "url": "https://github.com/e-im/GravityControl/issues"
              },
              {
                "id": 2,
                "name": "Source",
                "url": "https://github.com/e-im/GravityControl.git"
              },
              {
                "id": 3,
                "name": "Support",
                "url": "https://discord.gg/TNvq9y7esy"
              },
              {
                "id": 4,
                "name": "Wiki",
                "url": null
              }
            ]
          }
        ],
        "tags": [],
        "license": {
          "name": "Unspecified",
          "url": null,
          "type": "GPL"
        },
        "keywords": [],
        "sponsors": "",
        "donation": {
          "enable": false,
          "subject": ""
        }
      }
    },
    {
      "createdAt": "2022-12-23T20:25:56.75066Z",
      "name": "CoreProtect",
      "namespace": {
        "owner": "CORE",
        "slug": "CoreProtect"
      },
      "stats": {
        "views": 39268,
        "downloads": 12030,
        "recentViews": 4601,
        "recentDownloads": 1300,
        "stars": 138,
        "watchers": 87
      },
      "category": "admin_tools",
      "lastUpdated": "2024-05-14T00:47:11.845096Z",
      "visibility": "public",
      "avatarUrl": "https://hangarcdn.papermc.io/avatars/project/10.webp?v=1",
      "description": "CoreProtect is a fast, efficient, data logging and anti-griefing tool. Rollback and restore any amount of damage.",
      "userActions": {
        "starred": false,
        "watching": false,
        "flagged": false
      },
      "settings": {
        "links": [
          {
            "id": 0,
            "type": "top",
            "title": "top",
            "links": [
              {
                "id": 0,
                "name": "Homepage",
                "url": "https://coreprotect.net"
              },
              {
                "id": 1,
                "name": "Issues",
                "url": "https://github.com/PlayPro/CoreProtect/issues"
              },
              {
                "id": 2,
                "name": "Source",
                "url": "https://github.com/PlayPro/CoreProtect"
              },
              {
                "id": 3,
                "name": "Support",
                "url": "https://discord.gg/b4DZ4jy"
              },
              {
                "id": 4,
                "name": "Wiki",
                "url": "https://docs.coreprotect.net"
              }
            ]
          }
        ],
        "tags": [],
        "license": {
          "name": "Artistic License 2.0",
          "url": "https://github.com/PlayPro/CoreProtect/blob/master/LICENSE",
          "type": "Other"
        },
        "keywords": [
          "Rollback",
          "Restore",
          "Logging",
          "Grief",
          "Anti-Griefing"
        ],
        "sponsors": "A special thank you to everyone who supports the project on [Patreon](https://www.patreon.com/coreprotect).",
        "donation": {
          "enable": false,
          "subject": ""
        }
      }
    },
    {
      "createdAt": "2022-12-23T20:47:00.582094Z",
      "name": "ViaBackwards",
      "namespace": {
        "owner": "ViaVersion",
        "slug": "ViaBackwards"
      },
      "stats": {
        "views": 90866,
        "downloads": 45681,
        "recentViews": 12190,
        "recentDownloads": 6171,
        "stars": 102,
        "watchers": 70
      },
      "category": "misc",
      "lastUpdated": "2024-07-21T07:46:28.460498Z",
      "visibility": "public",
      "avatarUrl": "https://hangarcdn.papermc.io/avatars/project/12.webp?v=1",
      "description": "Allow clients with older versions to connect to your Minecraft server",
      "userActions": {
        "starred": false,
        "watching": false,
        "flagged": false
      },
      "settings": {
        "links": [
          {
            "id": 0,
            "type": "top",
            "title": "top",
            "links": [
              {
                "id": 4,
                "name": "Donate",
                "url": "https://github.com/sponsors/kennytv"
              },
              {
                "id": 1,
                "name": "Issues",
                "url": "https://github.com/ViaVersion/ViaBackwards/issues"
              },
              {
                "id": 2,
                "name": "Source",
                "url": "https://github.com/ViaVersion/ViaBackwards/"
              },
              {
                "id": 3,
                "name": "Support",
                "url": "https://discord.gg/viaversion"
              }
            ]
          },
          {
            "id": 1,
            "type": "sidebar",
            "title": "Early-Access builds",
            "links": [
              {
                "id": 0,
                "name": "GitHub Sponsors",
                "url": "https://github.com/sponsors/kennytv/sponsorships?sponsor=kennytv&tier_id=385613&preview=false"
              }
            ]
          }
        ],
        "tags": [
          "SUPPORTS_FOLIA",
          "ADDON"
        ],
        "license": {
          "name": "GPL",
          "url": "https://github.com/ViaVersion/ViaBackwards/blob/master/LICENSE",
          "type": "GPL"
        },
        "keywords": [
          "viaversion",
          "viabackwards",
          "protocolhack"
        ],
        "sponsors": "![https://www.yourkit.com/images/yklogo.png](https://www.yourkit.com/images/yklogo.png)\nYourKit supports open source projects with innovative and intelligent tools for monitoring and profiling Java and .NET applications. YourKit is the creator of [YourKit Java Profiler](https://www.yourkit.com/java/profiler/), [YourKit .NET Profiler](https://www.yourkit.com/.net/profiler/), and [YourKit YouMonitor](https://www.yourkit.com/youmonitor/).",
        "donation": {
          "enable": false,
          "subject": ""
        }
      }
    },
    {
      "createdAt": "2022-12-23T22:17:33.804966Z",
      "name": "MiniMOTD",
      "namespace": {
        "owner": "jmp",
        "slug": "MiniMOTD"
      },
      "stats": {
        "views": 22302,
        "downloads": 7657,
        "recentViews": 2300,
        "recentDownloads": 785,
        "stars": 55,
        "watchers": 38
      },
      "category": "admin_tools",
      "lastUpdated": "2024-06-24T02:57:06.520418Z",
      "visibility": "public",
      "avatarUrl": "https://hangarcdn.papermc.io/avatars/project/13.webp?v=1",
      "description": "Minecraft plugin/mod to set the server list MOTD using MiniMessage for formatting, supporting RGB colors",
      "userActions": {
        "starred": false,
        "watching": false,
        "flagged": false
      },
      "settings": {
        "links": [
          {
            "id": 0,
            "type": "top",
            "title": "top",
            "links": [
              {
                "id": 3,
                "name": "Discord",
                "url": "https://discord.gg/vBZSNAA"
              },
              {
                "id": 4,
                "name": "Wiki",
                "url": "https://github.com/jpenilla/MiniMOTD/wiki"
              },
              {
                "id": 2,
                "name": "Source",
                "url": "https://github.com/jpenilla/MiniMOTD"
              },
              {
                "id": 1,
                "name": "Issues",
                "url": "https://github.com/jpenilla/MiniMOTD/issues"
              }
            ]
          },
          {
            "id": 1,
            "type": "sidebar",
            "title": "Support the Developer",
            "links": [
              {
                "id": 0,
                "name": "GitHub Sponsors",
                "url": "https://github.com/sponsors/jpenilla/"
              }
            ]
          }
        ],
        "tags": [
          "SUPPORTS_FOLIA"
        ],
        "license": {
          "name": "MIT",
          "url": null,
          "type": "MIT"
        },
        "keywords": [
          "motd",
          "minimotd",
          "serverlist",
          "minimessage"
        ],
        "sponsors": "",
        "donation": {
          "enable": false,
          "subject": ""
        }
      }
    },
    {
      "createdAt": "2022-12-23T23:21:00.76682Z",
      "name": "Geyser",
      "namespace": {
        "owner": "GeyserMC",
        "slug": "Geyser"
      },
      "stats": {
        "views": 70807,
        "downloads": 19847,
        "recentViews": 7692,
        "recentDownloads": 2401,
        "stars": 135,
        "watchers": 69
      },
      "category": "misc",
      "lastUpdated": "2023-03-19T20:49:50.933106Z",
      "visibility": "public",
      "avatarUrl": "https://hangarcdn.papermc.io/avatars/project/14.webp?v=1",
      "description": "A bridge/proxy allowing you to connect to Minecraft: Java Edition servers with Minecraft: Bedrock Edition.",
      "userActions": {
        "starred": false,
        "watching": false,
        "flagged": false
      },
      "settings": {
        "links": [
          {
            "id": 0,
            "type": "top",
            "title": "top",
            "links": [
              {
                "id": 0,
                "name": "Homepage",
                "url": "https://geysermc.org"
              },
              {
                "id": 1,
                "name": "Issues",
                "url": "https://github.com/GeyserMC/Geyser/issues"
              },
              {
                "id": 2,
                "name": "Source",
                "url": "https://github.com/GeyserMC/Geyser"
              },
              {
                "id": 3,
                "name": "Support",
                "url": "https://discord.gg/GeyserMC"
              },
              {
                "id": 4,
                "name": "Wiki",
                "url": "https://wiki.geysermc.org"
              }
            ]
          }
        ],
        "tags": [
          "SUPPORTS_FOLIA"
        ],
        "license": {
          "name": "MIT",
          "url": "https://github.com/GeyserMC/Geyser/blob/master/LICENSE",
          "type": "MIT"
        },
        "keywords": [
          "java",
          "bedrock",
          "translator",
          "proxy",
          "protocol"
        ],
        "sponsors": "Thanks to all the donors over at [Open Collective](https://opencollective.com/geysermc) and [GitHub Sponsors](https://github.com/sponsors/GeyserMC).",
        "donation": {
          "enable": false,
          "subject": ""
        }
      }
    },
    {
      "createdAt": "2022-12-24T00:46:49.85121Z",
      "name": "LevelledMobs",
      "namespace": {
        "owner": "ArcanePlugins",
        "slug": "LevelledMobs"
      },
      "stats": {
        "views": 27777,
        "downloads": 4786,
        "recentViews": 3215,
        "recentDownloads": 451,
        "stars": 48,
        "watchers": 31
      },
      "category": "gameplay",
      "lastUpdated": "2024-07-03T00:41:59.80332Z",
      "visibility": "public",
      "avatarUrl": "https://hangarcdn.papermc.io/avatars/project/15.webp?v=1",
      "description": "Level-up mobs on your server, RPG-style! Perfect for Survival, Skyblock, and more.",
      "userActions": {
        "starred": false,
        "watching": false,
        "flagged": false
      },
      "settings": {
        "links": [
          {
            "id": 0,
            "type": "top",
            "title": "top",
            "links": [
              {
                "id": 0,
                "name": "Homepage",
                "url": "https://www.spigotmc.org/resources/levelledmobs.74304/"
              },
              {
                "id": 1,
                "name": "Issues",
                "url": "https://github.com/ArcanePlugins/LevelledMobs/issues/"
              },
              {
                "id": 2,
                "name": "Source",
                "url": "https://github.com/ArcanePlugins/LevelledMobs/"
              },
              {
                "id": 3,
                "name": "Support",
                "url": "https://discord.gg/HqZwdcJ"
              },
              {
                "id": 4,
                "name": "Wiki",
                "url": "https://arcaneplugins.gitbook.io/levelledmobs-the-ultimate-mob-levelling-solution"
              }
            ]
          }
        ],
        "tags": [
          "SUPPORTS_FOLIA"
        ],
        "license": {
          "name": "AGPL",
          "url": "https://github.com/ArcanePlugins/LevelledMobs/blob/master/LICENSE.md",
          "type": "AGPL"
        },
        "keywords": [
          "levelledmobs",
          "leveledmobs",
          "rpg",
          "leveling",
          "mobs"
        ],
        "sponsors": "*Thank you to Raid Shadow Legends for sponsoring this segment.* `/s`\n\nIf you are interested in sponsoring LevelledMobs, please let us know.",
        "donation": {
          "enable": false,
          "subject": ""
        }
      }
    },
    {
      "createdAt": "2022-12-24T00:51:20.466718Z",
      "name": "Carbon",
      "namespace": {
        "owner": "Vicarious",
        "slug": "Carbon"
      },
      "stats": {
        "views": 12916,
        "downloads": 1431,
        "recentViews": 1168,
        "recentDownloads": 142,
        "stars": 25,
        "watchers": 12
      },
      "category": "chat",
      "lastUpdated": "2024-07-05T17:12:28.41708Z",
      "visibility": "public",
      "avatarUrl": "https://hangarcdn.papermc.io/avatars/project/16.webp?v=1",
      "description": "Modern chat channel plugin using the MiniMessage format",
      "userActions": {
        "starred": false,
        "watching": false,
        "flagged": false
      },
      "settings": {
        "links": [
          {
            "id": 0,
            "type": "top",
            "title": "top",
            "links": [
              {
                "id": 1,
                "name": "Issues",
                "url": "https://github.com/Hexaoxide/Carbon/issues"
              },
              {
                "id": 2,
                "name": "Source",
                "url": "https://github.com/Hexaoxide/Carbon/"
              },
              {
                "id": 3,
                "name": "Support",
                "url": "https://discord.gg/S8s75Yf"
              },
              {
                "id": 4,
                "name": "Wiki",
                "url": "https://github.com/Hexaoxide/Carbon/wiki"
              }
            ]
          }
        ],
        "tags": [
          "SUPPORTS_FOLIA"
        ],
        "license": {
          "name": "GPL",
          "url": "https://github.com/Hexaoxide/Carbon/blob/2.1/LICENSE",
          "type": "GPL"
        },
        "keywords": [],
        "sponsors": "DemocracyCraft",
        "donation": {
          "enable": false,
          "subject": ""
        }
      }
    },
    {
      "createdAt": "2022-12-24T01:16:52.062218Z",
      "name": "Floodgate",
      "namespace": {
        "owner": "GeyserMC",
        "slug": "Floodgate"
      },
      "stats": {
        "views": 27993,
        "downloads": 8757,
        "recentViews": 3850,
        "recentDownloads": 1227,
        "stars": 71,
        "watchers": 50
      },
      "category": "misc",
      "lastUpdated": "2023-03-19T20:57:26.630238Z",
      "visibility": "public",
      "avatarUrl": "https://hangarcdn.papermc.io/avatars/project/17.webp?v=1",
      "description": "Hybrid mode plugin to allow for connections from Geyser to join online mode servers.",
      "userActions": {
        "starred": false,
        "watching": false,
        "flagged": false
      },
      "settings": {
        "links": [
          {
            "id": 0,
            "type": "top",
            "title": "top",
            "links": [
              {
                "id": 0,
                "name": "Homepage",
                "url": "https://geysermc.org"
              },
              {
                "id": 1,
                "name": "Issues",
                "url": "https://github.com/GeyserMC/Floodgate/issues"
              },
              {
                "id": 2,
                "name": "Source",
                "url": "https://github.com/GeyserMC/Floodgate"
              },
              {
                "id": 3,
                "name": "Support",
                "url": "https://discord.gg/geysermc"
              },
              {
                "id": 4,
                "name": "Wiki",
                "url": "https://wiki.geysermc.org/floodgate/"
              }
            ]
          }
        ],
        "tags": [
          "SUPPORTS_FOLIA"
        ],
        "license": {
          "name": "MIT",
          "url": "https://github.com/GeyserMC/Floodgate/blob/master/LICENSE",
          "type": "MIT"
        },
        "keywords": [
          "geyser",
          "floodgate",
          "auth",
          "authentication",
          "bedrock"
        ],
        "sponsors": "",
        "donation": {
          "enable": false,
          "subject": ""
        }
      }
    },
    {
      "createdAt": "2022-12-24T01:38:20.846113Z",
      "name": "Denizen",
      "namespace": {
        "owner": "DenizenScript",
        "slug": "Denizen"
      },
      "stats": {
        "views": 8898,
        "downloads": 721,
        "recentViews": 767,
        "recentDownloads": 115,
        "stars": 28,
        "watchers": 24
      },
      "category": "admin_tools",
      "lastUpdated": "2024-01-22T10:18:47.875433Z",
      "visibility": "public",
      "avatarUrl": "https://hangarcdn.papermc.io/avatars/project/18.webp?v=1",
      "description": "Scriptable Minecraft!",
      "userActions": {
        "starred": false,
        "watching": false,
        "flagged": false
      },
      "settings": {
        "links": [
          {
            "id": 0,
            "type": "top",
            "title": "top",
            "links": [
              {
                "id": 0,
                "name": "Homepage",
                "url": "https://denizenscript.com/"
              },
              {
                "id": 1,
                "name": "Issues",
                "url": null
              },
              {
                "id": 2,
                "name": "Source",
                "url": "https://github.com/DenizenScript/Denizen"
              },
              {
                "id": 3,
                "name": "Support",
                "url": "https://discord.gg/Q6pZGSR"
              },
              {
                "id": 4,
                "name": "Wiki",
                "url": "https://guide.denizenscript.com/"
              }
            ]
          }
        ],
        "tags": [],
        "license": {
          "name": null,
          "url": "https://github.com/DenizenScript/Denizen/blob/dev/LICENSE.txt",
          "type": "MIT"
        },
        "keywords": [
          "denizen",
          "script",
          "denizenscript"
        ],
        "sponsors": "",
        "donation": {
          "enable": false,
          "subject": ""
        }
      }
    },
    {
      "createdAt": "2022-12-24T04:46:25.164673Z",
      "name": "squaremap",
      "namespace": {
        "owner": "jmp",
        "slug": "squaremap"
      },
      "stats": {
        "views": 31774,
        "downloads": 5550,
        "recentViews": 3389,
        "recentDownloads": 640,
        "stars": 61,
        "watchers": 29
      },
      "category": "misc",
      "lastUpdated": "2024-07-18T02:42:20.895377Z",
      "visibility": "public",
      "avatarUrl": "https://hangarcdn.papermc.io/avatars/project/20.webp?v=1",
      "description": "squaremap is a minimalistic & lightweight world map viewer for Minecraft servers, using the vanilla map rendering style",
      "userActions": {
        "starred": false,
        "watching": false,
        "flagged": false
      },
      "settings": {
        "links": [
          {
            "id": 0,
            "type": "top",
            "title": "top",
            "links": [
              {
                "id": 3,
                "name": "Discord",
                "url": "https://discord.gg/PHpuzZS"
              },
              {
                "id": 4,
                "name": "Wiki",
                "url": "https://github.com/jpenilla/squaremap/wiki"
              },
              {
                "id": 2,
                "name": "Source",
                "url": "https://github.com/jpenilla/squaremap"
              },
              {
                "id": 1,
                "name": "Issues",
                "url": "https://github.com/jpenilla/squaremap/issues"
              }
            ]
          },
          {
            "id": 1,
            "type": "sidebar",
            "title": "Support the Developer",
            "links": [
              {
                "id": 0,
                "name": "GitHub Sponsors",
                "url": "https://github.com/sponsors/jpenilla/"
              }
            ]
          }
        ],
        "tags": [
          "SUPPORTS_FOLIA"
        ],
        "license": {
          "name": "MIT",
          "url": "https://github.com/jpenilla/squaremap/blob/master/LICENSE",
          "type": "MIT"
        },
        "keywords": [
          "map",
          "webmap",
          "render",
          "livemap",
          "worldmap"
        ],
        "sponsors": "",
        "donation": {
          "enable": false,
          "subject": ""
        }
      }
    },
    {
      "createdAt": "2022-12-24T12:51:31.538415Z",
      "name": "MineDown",
      "namespace": {
        "owner": "Phoenix616",
        "slug": "MineDown"
      },
      "stats": {
        "views": 2368,
        "downloads": 95,
        "recentViews": 140,
        "recentDownloads": 7,
        "stars": 6,
        "watchers": 1
      },
      "category": "dev_tools",
      "lastUpdated": "2023-01-20T21:48:25.490367Z",
      "visibility": "public",
      "avatarUrl": "https://hangarcdn.papermc.io/avatars/project/21.webp?v=1",
      "description": "A MarkDown inspired markup library for Minecraft chat components",
      "userActions": {
        "starred": false,
        "watching": false,
        "flagged": false
      },
      "settings": {
        "links": [
          {
            "id": 0,
            "type": "top",
            "title": "top",
            "links": [
              {
                "id": 0,
                "name": "Homepage",
                "url": null
              },
              {
                "id": 1,
                "name": "Issues",
                "url": "https://github.com/Phoenix616/MineDown/issues?q=is%3Aissue+is%3Aopen+sort%3Aupdated-desc"
              },
              {
                "id": 2,
                "name": "Source",
                "url": "https://github.com/Phoenix616/MineDown"
              },
              {
                "id": 3,
                "name": "Support",
                "url": null
              },
              {
                "id": 4,
                "name": "Wiki",
                "url": "https://wiki.phoenix616.dev/library/minedown/"
              }
            ]
          }
        ],
        "tags": [],
        "license": {
          "name": "MIT",
          "url": "https://github.com/Phoenix616/MineDown/blob/master/LICENSE",
          "type": "MIT"
        },
        "keywords": [
          "minedown",
          "markdown",
          "formatting",
          "components",
          "colors"
        ],
        "sponsors": "GitHub Sponsors: https://github.com/sponsors/Phoenix616\n\nKo-Fi Donation Feed: https://ko-fi.com/Phoenix616",
        "donation": {
          "enable": false,
          "subject": ""
        }
      }
    },
    {
      "createdAt": "2022-12-24T14:22:07.664521Z",
      "name": "BuycraftX-Velocity",
      "namespace": {
        "owner": "FivePB",
        "slug": "BuycraftX-Velocity"
      },
      "stats": {
        "views": 2228,
        "downloads": 350,
        "recentViews": 85,
        "recentDownloads": 3,
        "stars": 1,
        "watchers": 0
      },
      "category": "admin_tools",
      "lastUpdated": "2022-12-24T14:46:15.710031Z",
      "visibility": "public",
      "avatarUrl": "https://hangarcdn.papermc.io/avatars/project/22.webp?v=1",
      "description": "A modified version of BuycraftX for modern Velocity versions",
      "userActions": {
        "starred": false,
        "watching": false,
        "flagged": false
      },
      "settings": {
        "links": [
          {
            "id": 0,
            "type": "top",
            "title": "top",
            "links": [
              {
                "id": 0,
                "name": "Homepage",
                "url": "https://www.tebex.io/"
              },
              {
                "id": 1,
                "name": "Issues",
                "url": "https://github.com/Xernium/BuycraftX-Velocity/issues"
              },
              {
                "id": 2,
                "name": "Source",
                "url": "https://github.com/Xernium/BuycraftX-Velocity"
              },
              {
                "id": 3,
                "name": "Support",
                "url": null
              },
              {
                "id": 4,
                "name": "Wiki",
                "url": null
              }
            ]
          }
        ],
        "tags": [],
        "license": {
          "name": null,
          "url": "https://github.com/Xernium/BuycraftX-Velocity/blob/master/LICENSE.md",
          "type": "MIT"
        },
        "keywords": [
          "buycraft",
          "buycraftx"
        ],
        "sponsors": "",
        "donation": {
          "enable": false,
          "subject": ""
        }
      }
    },
    {
      "createdAt": "2022-12-24T16:00:15.69127Z",
      "name": "Essentials",
      "namespace": {
        "owner": "EssentialsX",
        "slug": "Essentials"
      },
      "stats": {
        "views": 84302,
        "downloads": 27254,
        "recentViews": 9501,
        "recentDownloads": 2934,
        "stars": 149,
        "watchers": 81
      },
      "category": "admin_tools",
      "lastUpdated": "2023-08-06T10:41:48.751944Z",
      "visibility": "public",
      "avatarUrl": "https://hangarcdn.papermc.io/avatars/project/23.webp?v=2",
      "description": "The essential plugin suite for Paper! (and Spigot)",
      "userActions": {
        "starred": false,
        "watching": false,
        "flagged": false
      },
      "settings": {
        "links": [
          {
            "id": 0,
            "type": "top",
            "title": "top",
            "links": [
              {
                "id": 2,
                "name": "Source",
                "url": "https://github.com/EssentialsX/Essentials"
              },
              {
                "id": 1,
                "name": "Issues",
                "url": "https://github.com/EssentialsX/Essentials/issues"
              },
              {
                "id": 3,
                "name": "Support",
                "url": "https://essentialsx.net/community.html"
              },
              {
                "id": 4,
                "name": "Wiki",
                "url": "https://essentialsx.net/wiki/Home.html"
              }
            ]
          },
          {
            "id": 2,
            "type": "sidebar",
            "title": "🛟 Help!",
            "links": [
              {
                "id": 0,
                "name": "Get help on MOSS Discord",
                "url": "https://discord.gg/h8CnPSw"
              },
              {
                "id": 1,
                "name": "Report bugs on GitHub",
                "url": "https://github.com/EssentialsX/Essentials/issues/new/choose"
              },
              {
                "id": 2,
                "name": "API documentation",
                "url": "https://jd-v2.essentialsx.net/"
              }
            ]
          },
          {
            "id": 1,
            "type": "sidebar",
            "title": "💬 Community",
            "links": [
              {
                "id": 3,
                "name": "Request a feature",
                "url": "https://github.com/EssentialsX/Essentials/issues/new/choose"
              },
              {
                "id": 2,
                "name": "GitHub Discussions forum",
                "url": "https://github.com/EssentialsX/Essentials/discussions/"
              },
              {
                "id": 1,
                "name": "Development Discord",
                "url": "https://discord.gg/bSrXB43nW7"
              }
            ]
          }
        ],
        "tags": [],
        "license": {
          "name": "GPL",
          "url": "https://github.com/EssentialsX/Essentials/blob/2.x/LICENSE",
          "type": "GPL"
        },
        "keywords": [
          "essentials",
          "essentialsx",
          "essx",
          "bukkit",
          "spigot"
        ],
        "sponsors": "",
        "donation": {
          "enable": false,
          "subject": ""
        }
      }
    },
    {
      "createdAt": "2022-12-25T17:09:45.591501Z",
      "name": "EntityCount",
      "namespace": {
        "owner": "SlimeDog",
        "slug": "EntityCount"
      },
      "stats": {
        "views": 1594,
        "downloads": 95,
        "recentViews": 85,
        "recentDownloads": 2,
        "stars": 2,
        "watchers": 0
      },
      "category": "admin_tools",
      "lastUpdated": "2023-12-10T01:03:06.102766Z",
      "visibility": "public",
      "avatarUrl": "https://hangarcdn.papermc.io/avatars/project/24.webp?v=4",
      "description": "Count entities in a world",
      "userActions": {
        "starred": false,
        "watching": false,
        "flagged": false
      },
      "settings": {
        "links": [
          {
            "id": 0,
            "type": "top",
            "title": "top",
            "links": [
              {
                "id": 1,
                "name": "Issues",
                "url": "https://github.com/SlimeDog/EntityCount/issues?q=is%3Aissue+is%3Aopen+sort%3Aupdated-desc"
              },
              {
                "id": 2,
                "name": "Source",
                "url": "https://github.com/SlimeDog/EntityCount"
              }
            ]
          },
          {
            "id": 1,
            "type": "sidebar",
            "title": "Pizza & Beer",
            "links": [
              {
                "id": 0,
                "name": "PayPal",
                "url": "https://paypal.me/RAMcIntosh"
              }
            ]
          }
        ],
        "tags": [],
        "license": {
          "name": "GPL",
          "url": "https://github.com/SlimeDog/EntityCount/blob/master/LICENSE",
          "type": "GPL"
        },
        "keywords": [
          "entity",
          "count"
        ],
        "sponsors": "",
        "donation": {
          "enable": false,
          "subject": ""
        }
      }
    },
    {
      "createdAt": "2022-12-25T18:49:49.854273Z",
      "name": "NetworkInterceptor",
      "namespace": {
        "owner": "SlimeDog",
        "slug": "NetworkInterceptor"
      },
      "stats": {
        "views": 2278,
        "downloads": 108,
        "recentViews": 112,
        "recentDownloads": 3,
        "stars": 2,
        "watchers": 2
      },
      "category": "admin_tools",
      "lastUpdated": "2023-12-10T01:18:39.300237Z",
      "visibility": "public",
      "avatarUrl": "https://hangarcdn.papermc.io/avatars/project/25.webp?v=5",
      "description": "Monitor and block outgoing network connections",
      "userActions": {
        "starred": false,
        "watching": false,
        "flagged": false
      },
      "settings": {
        "links": [
          {
            "id": 0,
            "type": "top",
            "title": "top",
            "links": [
              {
                "id": 1,
                "name": "Issues",
                "url": "https://github.com/SlimeDog/NetworkInterceptor/issues/?q=is%3Aissue+is%3Aopen+sort%3Aupdated-desc"
              },
              {
                "id": 2,
                "name": "Source",
                "url": "https://github.com/SlimeDog/NetworkInterceptor/"
              },
              {
                "id": 4,
                "name": "Wiki",
                "url": "https://github.com/SlimeDog/NetworkInterceptor/wiki"
              }
            ]
          },
          {
            "id": 1,
            "type": "sidebar",
            "title": "Pizza & Beer",
            "links": [
              {
                "id": 0,
                "name": "PayPal",
                "url": "https://paypal.me/RAMcIntosh"
              }
            ]
          }
        ],
        "tags": [],
        "license": {
          "name": "Unlicense",
          "url": "https://github.com/SlimeDog/NetworkInterceptor/blob/master/LICENSE.txt",
          "type": "Other"
        },
        "keywords": [
          "network",
          "intercept",
          "interceptor",
          "block",
          "traffic"
        ],
        "sponsors": "",
        "donation": {
          "enable": false,
          "subject": ""
        }
      }
    },
    {
      "createdAt": "2022-12-25T19:08:39.988924Z",
      "name": "BiomeRemap",
      "namespace": {
        "owner": "SlimeDog",
        "slug": "BiomeRemap"
      },
      "stats": {
        "views": 2664,
        "downloads": 77,
        "recentViews": 126,
        "recentDownloads": 5,
        "stars": 2,
        "watchers": 2
      },
      "category": "world_management",
      "lastUpdated": "2023-12-10T00:51:09.725076Z",
      "visibility": "public",
      "avatarUrl": "https://hangarcdn.papermc.io/avatars/project/26.webp?v=2",
      "description": "Remap biomes to create strange new worlds",
      "userActions": {
        "starred": false,
        "watching": false,
        "flagged": false
      },
      "settings": {
        "links": [
          {
            "id": 0,
            "type": "top",
            "title": "top",
            "links": [
              {
                "id": 1,
                "name": "Issues",
                "url": "https://github.com/SlimeDog/BiomeRemap/issues?q=is%3Aissue+is%3Aopen+sort%3Aupdated-desc"
              },
              {
                "id": 2,
                "name": "Source",
                "url": "https://github.com/SlimeDog/BiomeRemap/"
              },
              {
                "id": 4,
                "name": "Wiki",
                "url": "https://github.com/SlimeDog/BiomeRemap/wiki/"
              }
            ]
          },
          {
            "id": 1,
            "type": "sidebar",
            "title": "Pizza & Beer",
            "links": [
              {
                "id": 0,
                "name": "PayPal",
                "url": "https://paypal.me/RAMcIntosh"
              }
            ]
          }
        ],
        "tags": [],
        "license": {
          "name": "GPL",
          "url": "https://github.com/SlimeDog/BiomeRemap/blob/master/LICENSE",
          "type": "GPL"
        },
        "keywords": [
          "biome",
          "remap"
        ],
        "sponsors": "",
        "donation": {
          "enable": false,
          "subject": ""
        }
      }
    },
    {
      "createdAt": "2022-12-25T19:27:15.175571Z",
      "name": "MobColors",
      "namespace": {
        "owner": "SlimeDog",
        "slug": "MobColors"
      },
      "stats": {
        "views": 1316,
        "downloads": 34,
        "recentViews": 81,
        "recentDownloads": 1,
        "stars": 1,
        "watchers": 2
      },
      "category": "world_management",
      "lastUpdated": "2023-12-10T01:08:57.410814Z",
      "visibility": "public",
      "avatarUrl": "https://hangarcdn.papermc.io/avatars/project/27.webp?v=4",
      "description": "Manage mob colors and variants",
      "userActions": {
        "starred": false,
        "watching": false,
        "flagged": false
      },
      "settings": {
        "links": [
          {
            "id": 0,
            "type": "top",
            "title": "top",
            "links": [
              {
                "id": 1,
                "name": "Issues",
                "url": "https://github.com/SlimeDog/MobColors/issues/?q=is%3Aissue+is%3Aopen+sort%3Aupdated-desc"
              },
              {
                "id": 2,
                "name": "Source",
                "url": "https://github.com/SlimeDog/MobColors/"
              },
              {
                "id": 4,
                "name": "Wiki",
                "url": "https://github.com/SlimeDog/MobColors/wiki"
              }
            ]
          },
          {
            "id": 1,
            "type": "sidebar",
            "title": "Pizza & Beer",
            "links": [
              {
                "id": 0,
                "name": "PayPal",
                "url": "https://paypal.me/RAMcIntosh"
              }
            ]
          }
        ],
        "tags": [],
        "license": {
          "name": "GPL",
          "url": "https://github.com/SlimeDog/MobColors/blob/master/LICENSE",
          "type": "GPL"
        },
        "keywords": [
          "mob",
          "entity",
          "color"
        ],
        "sponsors": "",
        "donation": {
          "enable": false,
          "subject": ""
        }
      }
    },
    {
      "createdAt": "2022-12-25T19:39:46.120246Z",
      "name": "pHD",
      "namespace": {
        "owner": "SlimeDog",
        "slug": "pHD"
      },
      "stats": {
        "views": 1353,
        "downloads": 30,
        "recentViews": 66,
        "recentDownloads": 0,
        "stars": 1,
        "watchers": 1
      },
      "category": "world_management",
      "lastUpdated": "2023-06-15T23:51:33.327521Z",
      "visibility": "public",
      "avatarUrl": "https://hangarcdn.papermc.io/avatars/project/28.webp?v=4",
      "description": "Manage holograms with intelligence",
      "userActions": {
        "starred": false,
        "watching": false,
        "flagged": false
      },
      "settings": {
        "links": [
          {
            "id": 0,
            "type": "top",
            "title": "top",
            "links": [
              {
                "id": 1,
                "name": "Issues",
                "url": "https://github.com/SlimeDog/pHD/issues/?q=is%3Aissue+is%3Aopen+sort%3Aupdated-desc"
              },
              {
                "id": 2,
                "name": "Source",
                "url": "https://github.com/SlimeDog/pHD/"
              },
              {
                "id": 4,
                "name": "Wiki",
                "url": "https://github.com/SlimeDog/pHD/wiki"
              }
            ]
          },
          {
            "id": 1,
            "type": "sidebar",
            "title": "Pizza & Beer",
            "links": [
              {
                "id": 0,
                "name": "PayPal",
                "url": "https://paypal.me/RAMcIntosh"
              }
            ]
          }
        ],
        "tags": [],
        "license": {
          "name": "GPL",
          "url": "https://github.com/SlimeDog/pHD/blob/master/LICENSE",
          "type": "GPL"
        },
        "keywords": [],
        "sponsors": "",
        "donation": {
          "enable": false,
          "subject": ""
        }
      }
    }
  ]
}
        "#;

    let projects = from_str(raw);

    assert!(projects.is_ok());

    let _projects: HangarProjects = projects.unwrap();
  }
}
