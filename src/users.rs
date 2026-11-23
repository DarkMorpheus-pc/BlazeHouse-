use crate::capabilities::{has_cmd, run_cmd};
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserAccount {
    pub username: String,
    pub uid: u32,
    pub gid: u32,
    pub primary_group: String,
    pub supplementary_groups: Vec<String>,
    pub home_dir: String,
    pub shell: String,
    pub is_admin: bool,
    pub is_current: bool,
}

pub fn current_user() -> UserAccount {
    let username = env::var("USER").unwrap_or_else(|_| "current_user".to_string());
    let home_dir = env::var("HOME").unwrap_or_else(|_| "/home".to_string());
    let shell = env::var("SHELL").unwrap_or_else(|_| "/bin/bash".to_string());

    let mut uid = 1000;
    let mut gid = 1000;
    let mut primary_group = String::from("users");
    let mut groups = Vec::new();
    let mut is_admin = false;

    if has_cmd("id") {
        let (ok_u, out_u) = run_cmd("id", &["-u"]);
        if ok_u {
            if let Ok(u) = out_u.trim().parse::<u32>() {
                uid = u;
            }
        }

        let (ok_g, out_g) = run_cmd("id", &["-g"]);
        if ok_g {
            if let Ok(g) = out_g.trim().parse::<u32>() {
                gid = g;
            }
        }

        let (ok_gn, out_gn) = run_cmd("id", &["-gn"]);
        if ok_gn {
            primary_group = out_gn.trim().to_string();
        }

        let (ok_gr, out_gr) = run_cmd("id", &["-Gn"]);
        if ok_gr {
            groups = out_gr.split_whitespace().map(|s| s.to_string()).collect();
            if groups
                .iter()
                .any(|g| g == "sudo" || g == "wheel" || g == "adm" || g == "admin" || g == "root")
            {
                is_admin = true;
            }
        }
    }

    UserAccount {
        username,
        uid,
        gid,
        primary_group,
        supplementary_groups: groups,
        home_dir,
        shell,
        is_admin,
        is_current: true,
    }
}

pub fn parse_user_groups_and_admin(
    group_content: &str,
    username: &str,
    primary_gid: u32,
) -> (String, Vec<String>, bool) {
    let mut primary_group = String::from("users");
    let mut supp_groups = Vec::new();
    let mut is_admin = false;
    let admin_groups = ["wheel", "sudo", "adm", "admin", "root"];

    for line in group_content.lines() {
        let parts: Vec<&str> = line.split(':').collect();
        if parts.len() >= 4 {
            let gname = parts[0].trim();
            let gid = parts[2].trim().parse::<u32>().unwrap_or(u32::MAX);
            let members: Vec<&str> = parts[3]
                .split(',')
                .map(|m| m.trim())
                .filter(|m| !m.is_empty())
                .collect();

            let is_primary = gid == primary_gid;
            if is_primary {
                primary_group = gname.to_string();
            }

            let is_member = is_primary || members.contains(&username);
            if is_member {
                if !supp_groups.iter().any(|g| g == gname) {
                    supp_groups.push(gname.to_string());
                }
                if admin_groups.contains(&gname) {
                    is_admin = true;
                }
            }
        }
    }

    (primary_group, supp_groups, is_admin)
}

pub fn list_system_human_users() -> Vec<UserAccount> {
    let curr = current_user();
    let mut list = vec![curr.clone()];
    let group_content = fs::read_to_string("/etc/group").unwrap_or_default();

    if let Ok(content) = fs::read_to_string("/etc/passwd") {
        for line in content.lines() {
            let parts: Vec<&str> = line.split(':').collect();
            if parts.len() >= 7 {
                let uname = parts[0];
                let u_uid = parts[2].parse::<u32>().unwrap_or(0);
                let u_gid = parts[3].parse::<u32>().unwrap_or(0);
                let u_home = parts[5].to_string();
                let u_shell = parts[6].to_string();

                if (1000..65000).contains(&u_uid) && uname != curr.username && uname != "nobody" {
                    let (primary_group, supplementary_groups, is_admin) =
                        parse_user_groups_and_admin(&group_content, uname, u_gid);
                    list.push(UserAccount {
                        username: uname.to_string(),
                        uid: u_uid,
                        gid: u_gid,
                        primary_group,
                        supplementary_groups,
                        home_dir: u_home,
                        shell: u_shell,
                        is_admin,
                        is_current: false,
                    });
                }
            }
        }
    }

    list
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_current_user() {
        let u = current_user();
        assert!(!u.username.is_empty());
        assert!(!u.home_dir.is_empty());
    }

    #[test]
    fn test_user_admin_detection_from_etc_group() {
        let sample_group = "\
root:x:0:
wheel:x:10:alice,darkmorpheus
adm:x:4:bob
users:x:100:
alice:x:1001:
bob:x:1002:
charlie:x:1003:
";
        let (pg_a, groups_a, admin_a) = parse_user_groups_and_admin(sample_group, "alice", 1001);
        assert_eq!(pg_a, "alice");
        assert!(groups_a.contains(&"wheel".to_string()));
        assert!(admin_a);

        let (_pg_b, groups_b, admin_b) = parse_user_groups_and_admin(sample_group, "bob", 1002);
        assert!(groups_b.contains(&"adm".to_string()));
        assert!(admin_b);

        let (pg_c, _groups_c, admin_c) = parse_user_groups_and_admin(sample_group, "charlie", 1003);
        assert_eq!(pg_c, "charlie");
        assert!(!admin_c);
    }
}
