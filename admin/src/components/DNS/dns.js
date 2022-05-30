import React, { useState, useEffect } from "react";
import {
  Layout,
  Col,
  Row,
  Form,
  Table,
  Checkbox,
  Tag,
  message,
  Popconfirm,
  Divider,
  Input,
  Space,
  Button,
  Tabs,
} from "antd";
import { IoIosHelpCircle } from "react-icons/io";
import { Link } from "react-router-dom";
import axios from "axios";
import { ControlOutlined, ImportOutlined } from "@ant-design/icons";
import Cookies from "js-cookie";

const { Content } = Layout;
const { TabPane } = Tabs;

const DNSSetting = () => {
  // ------token ------
  const baseUrl = process.env.REACT_APP_API_URL;
  // const getToken = localStorage.getItem("token");
  const getToken = Cookies.get("token");
  const auth = {
    Authorization: "Bearer " + getToken,
  };
  // --------state ---------

  const [, setLoading] = useState(false);
  const [datas, setDatas] = useState([]);
  const [zones, setZones] = useState("internal");
  const [form] = Form.useForm();

  function callback(key) {
    setZones(key);
  }

  // ----------get data -------------

  async function fetchData() {
    await axios({
      method: "GET",
      url: `${baseUrl}/settings/dns/status/${zones}`,
      headers: {
        "content-type": "application/json",
        ...auth,
      },
    })
      .then((res) => {
        setDatas(res.data);
        setTimeout(() => {
          setLoading(false);
        }, 1000);
      })
      .catch((err) => console.log(err));
  }

  useEffect(() => {
    fetchData();
  }, [zones]);

  //  --------delete record ----------

  const handleDelete = async (domain_name) => {
    console.log(domain_name);
    await axios
      .delete(`${baseUrl}/settings/dns/delete/${zones}/${domain_name}`, {
        headers: {
          "content-type": "application/json",
          ...auth,
        },
      })
      .then((res) => {
        if ((res.statusCode = 200)) {
          message.success("Successful!");
          fetchData();
          setLoading(false);
        } else {
          setTimeout(() => {
            message.error("Operation Failed! ");
          }, 1000);
        }
      })
      .catch((err) => {
        setLoading(false);
        message.error(err.response.data.reason);
      });
  };

  // ------- apply button ---------

  const handleApply = async (data) => {
    const inputData = {
      domain_name: data.domain_name,
      status: false,
      zone_record: null,
    };
    console.log(inputData);
    await axios
      .post(`${baseUrl}/settings/dns/new/${zones}`, inputData, {
        headers: {
          "content-type": "application/json",
          ...auth,
        },
      })

      .then((res) => {
        if ((res.statusCode = 200)) {
          message.success("Successful!");
          form.resetFields();
          fetchData();
          setLoading(false);
        } else {
          message.error("Operation Failed! ");
        }
      })

      .catch((err) => {
        message.error("Operation Failed! ");
        console.log(err);
      });
  };

  const columns = [
    {
      title: "Name",
      dataIndex: "domain_name",
      key: "domain_name",
      width: "60%",
    },
    {
      title: "Status",
      dataIndex: "status",
      width: "20%",
      key: "status",
      render: (status) => {
        return <Checkbox disabled checked={status} />;
      },
    },
    {
      title: "Actions",
      width: "20%",
      dataIndex: "id",
      render: (id, data) => {
        const { domain_name } = data;
        return (
          <React.Fragment>
            <Link to={`/dns-management/${zones}/${domain_name}`}>
              <Tag color="processing">Control</Tag>
            </Link>
            <Divider type="vertical" />
            <Popconfirm
              placement="top"
              title={
                <span>
                  Are you sure to delete <b>{domain_name} </b>?
                </span>
              }
              okText="Yes"
              cancelText="No"
              onConfirm={() => handleDelete(domain_name)}
            >
              <Tag color="error" style={{ cursor: "pointer" }}>
                Delete
              </Tag>
            </Popconfirm>
          </React.Fragment>
        );
      },
    },
  ];

  // -------------- create domain name ----------------

  // const CreateDomain = () => {
  //   return (
  //     <React.Fragment>
  //       <Form form={form} layout="inline" onFinish={handleApply}>
  //         <Form.Item
  //           label="Domain name"
  //           name="domain_name"
  //           rules={[{ required: true, message: "Input domain name!" }]}
  //         >
  //           <Input
  //             placeholder="example.com "
  //             size="large"
  //             className="input-info-dns"
  //           />
  //         </Form.Item>
  //         <Form.Item>
  //           <Button
  //             type="primary"
  //             htmlType="submit"
  //             className="button-update2"
  //             size="large"
  //           >
  //             Create
  //           </Button>
  //         </Form.Item>
  //       </Form>
  //     </React.Fragment>
  //   );
  // };

  return (
    <React.Fragment>
      <Content>
        <Row gutter={12}>
          <Col span={16}>
            <div className="card">
              <div className="container">
                <div className="container-header">
                  <h1>DNS MANAGEMENT</h1>
                </div>

                <Tabs defaultActiveKey="internal" onChange={callback}>
                  <TabPane
                    tab={
                      <span>
                        <ControlOutlined />
                        INTERNAL
                      </span>
                    }
                    key="internal"
                  >
                    <div className="dns-desc-container">
                      <Form form={form} layout="inline" onFinish={handleApply}>
                        <Form.Item
                          label="Domain name"
                          name="domain_name"
                          rules={[
                            { required: true, message: "Input domain name!" },
                          ]}
                        >
                          <Input
                            placeholder="example.com "
                            size="large"
                            className="input-info-dns"
                          />
                        </Form.Item>
                        <Form.Item>
                          <Button
                            type="primary"
                            htmlType="submit"
                            className="button-update2"
                            size="large"
                          >
                            Create
                          </Button>
                        </Form.Item>
                      </Form>
                    </div>
                    <Form>
                      <div className="dns-desc-container">
                        <Form.Item>
                          <Table
                            columns={columns}
                            dataSource={datas}
                            pagination={false}
                            scroll={{ y: 600 }}
                          />
                        </Form.Item>
                      </div>
                    </Form>
                  </TabPane>
                  <TabPane
                    tab={
                      <span>
                        <ImportOutlined />
                        EXTERNAL
                      </span>
                    }
                    key="external"
                  >
                    <div className="dns-desc-container">
                      <Form form={form} layout="inline" onFinish={handleApply}>
                        <Form.Item
                          label="Domain name"
                          name="domain_name"
                          rules={[
                            { required: true, message: "Input domain name!" },
                          ]}
                        >
                          <Input
                            placeholder="example.com "
                            size="large"
                            className="input-info-dns"
                          />
                        </Form.Item>
                        <Form.Item>
                          <Button
                            type="primary"
                            htmlType="submit"
                            className="button-update2"
                            size="large"
                          >
                            Create
                          </Button>
                        </Form.Item>
                      </Form>
                    </div>
                    <Form>
                      <div className="dns-desc-container">
                        <Form.Item>
                          <Table
                            columns={columns}
                            dataSource={datas}
                            pagination={false}
                            scroll={{ y: 600 }}
                          />
                        </Form.Item>
                      </div>
                    </Form>
                  </TabPane>
                </Tabs>
              </div>
            </div>
          </Col>
          <Col span={8}>
            <div className="card2">
              <div className="container">
                <div className="container-header">
                  <Space>
                    <h1>HELPS</h1>
                    <IoIosHelpCircle className="icon-help" />
                  </Space>
                </div>
                <>
                  <p>
                    The Domain Name System (DNS) is the phonebook of the
                    Internet. Humans access information online through domain
                    names, like{" "}
                    <a
                      target="_blank"
                      rel="noopener noreferrer"
                      href="https://www.nytimes.com"
                    >
                      nytimes.com
                    </a>{" "}
                    or{" "}
                    <a
                      target="_blank"
                      rel="noopener noreferrer"
                      href="https://www.espn.com"
                    >
                      espn.com
                    </a>
                    . Web browsers interact through Internet Protocol (IP)
                    addresses.
                  </p>
                  <p>
                    {" "}
                    DNS translates domain names to IP addresses so browsers can
                    load Internet resources.
                  </p>
                  <p>
                    A domain name is a string that identifies a realm of
                    administrative autonomy, authority or control within the
                    Internet. Domain names are used in various networking
                    contexts and for application-specific naming and addressing
                    purposes. Example:{" "}
                    <a
                      target="_blank"
                      rel="noopener noreferrer"
                      href="https://www.koompi.com"
                    >
                      koompi.com
                    </a>
                    . Domain name does not allow space or slashes
                  </p>
                  <p>
                    In DNS management, you can disable or enable a DNS entry
                    that you created with the button 'status'.{" "}
                  </p>
                  <p>
                    Control button allow you to add changes to a DNS domain name
                    entry's records inside.{" "}
                  </p>
                  <p>Delete button allow you to remove a DNS entry</p>
                </>
              </div>
            </div>
          </Col>
        </Row>
      </Content>
    </React.Fragment>
  );
};

export default DNSSetting;
