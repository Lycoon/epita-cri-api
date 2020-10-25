mod models;

use models::Campus;
use models::CRIGroup;
use models::CRIComputedMembership;
use models::SimpleCRIUser;
use models::Profile;
use models::CRIUser;
use reqwest::blocking::Client;

const BASE_URL: &str = "https://cri.epita.fr/api/v2";

#[derive(serde::Deserialize)]
struct CampusList(Vec<Campus>);

#[derive(serde::Deserialize)]
struct CRIGroupList(Vec<CRIGroup>);

#[derive(serde::Deserialize)]
struct CRIComputedMembershipList(Vec<CRIComputedMembership>);

#[derive(serde::Deserialize)]
struct SimpleCRIUserList(Vec<SimpleCRIUser>);

#[derive(serde::Deserialize)]
struct CRIUserList(Vec<CRIUser>);

#[derive(serde::Deserialize)]
struct ProfileList(Vec<Profile>);

fn make_api_call(client: &Client, endpoint: &str) -> String
{
    client.get(&format!("{}/{}", BASE_URL, endpoint))
    .header("authorization", "Basic token")
    .send()
    .unwrap()
    .text()
    .unwrap()
}

/*
* Campus
*/
fn get_campus_list(client: &Client) -> Vec<Campus>
{
    let campus_list: CampusList = serde_json::from_str(&make_api_call(client, "campus")).unwrap();
    campus_list.0
}

fn get_campus(client: &Client, slug: &str) -> Campus
{
    serde_json::from_str(&make_api_call(client, &format!("campus/{}", slug))).unwrap()
}

/*
* Groups
*/
fn get_groups_list(client: &Client, args: Vec<&str>) -> Vec<CRIGroup>
{
    let groups_list: CRIGroupList = serde_json::from_str(&make_api_call(client, &format!("groups/{}", build_args(args)))).unwrap();
    groups_list.0
}

fn get_group(client: &Client, slug: &str) -> CRIGroup
{
    serde_json::from_str(&make_api_call(client, &format!("groups/{}", slug))).unwrap()
}

fn get_group_history(client: &Client, slug: &str) -> Vec<CRIComputedMembership>
{
    let group_history: CRIComputedMembershipList = serde_json::from_str(&make_api_call(client, &format!("groups/{}/history", slug))).unwrap();
    group_history.0
}

fn get_group_members(client: &Client, slug: &str) -> Vec<SimpleCRIUser>
{
    let group_members: SimpleCRIUserList = serde_json::from_str(&make_api_call(client, &format!("groups/{}/members", slug))).unwrap();
    group_members.0
}

/*
* Users
*/
fn get_users(client: &Client) -> Vec<CRIUser>
{
    let users: CRIUserList = serde_json::from_str(&make_api_call(client, "users")).unwrap();
    users.0
}

fn get_user_me(client: &Client) -> Vec<Profile>
{
    let user_me: ProfileList = serde_json::from_str(&make_api_call(client, "users/me/")).unwrap();
    user_me.0
}

fn get_user_search(client: &Client, args: Vec<&str>) -> Vec<CRIUser>
{
    let user_search: CRIUserList = serde_json::from_str(&make_api_call(client, &format!("users/search/{}", build_args(args)))).unwrap();
    user_search.0
}

fn get_user(client: &Client, login: &str) -> CRIUser
{
    serde_json::from_str(&make_api_call(client, &format!("users/{}/", login))).unwrap()
}

fn build_args(args: Vec<&str>) -> String
{
    let mut str = String::new();
    if args.iter().len() > 0{
        str += &format!("?{:?}", args.iter().nth(0));
    }
    else{
        return String::new();
    }

    if args.iter().len() > 1{
        for arg in args.iter(){
            str += &format!("&{}", arg);
        }
    }
    str
}
