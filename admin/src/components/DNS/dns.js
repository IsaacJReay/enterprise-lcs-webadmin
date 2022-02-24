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
  Button,
} from "antd";

import { Link } from "react-router-dom";
import axios from "axios";

const { Content } = Layout;

const DNSSetting = () => {
  // ------token ------
  const baseUrl = process.env.REACT_APP_API_URL;
  const getToken = localStorage.getItem("token");
  const auth = {
    Authorization: "Bearer " + getToken,
  };
  // --------state ---------

  const [, setLoading] = useState(false);
  const [datas, setDatas] = useState([]);
  const [form] = Form.useForm();

  // ----------get data -------------

  async function fetchData() {
    await axios({
      method: "GET",
      url: `${baseUrl}/settings/dns/domain_name/status`,
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
  }, []);

  //  --------delete record ----------

  const handleDelete = async (id) => {
    await axios
      .delete(`${baseUrl}/settings/dns/domain_name/deletion`, {
        headers: {
          "content-type": "application/json",
          ...auth,
        },
        data: { id: id },
      })
      .then((res) => {
        if (res.data.operation_status === "Success") {
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
    };
    await axios
      .post(`${baseUrl}/settings/dns/domain_name/creation`, inputData, {
        headers: {
          "content-type": "application/json",
          ...auth,
        },
      })

      .then((res) => {
        if (res.data.operation_status === "Success") {
          message.success("Successful!");
          form.resetFields();
          fetchData();
          setLoading(false);
        } else {
          setLoading(true);
          message.error("Operation Failed! ");
          setLoading(false);
        }
      })

      .catch((err) => {
        setTimeout(() => {
          setLoading(false);
        }, 1000);
        message.error(err.response.data.reason);
      });
  };

  const columns = [
    {
      title: "No",
      width: "10%",
      dataIndex: "id",
      key: "id",
    },
    {
      title: "Name",
      dataIndex: "domain_name",
      key: "domain_name",
    },
    {
      title: "Status",
      dataIndex: "status",
      key: "status",
      render: (status) => {
        return <Checkbox disabled checked={status} />;
      },
    },
    {
      title: "Actions",
      dataIndex: "id",
      render: (id, data) => {
        const { domain_name } = data;
        return (
          <React.Fragment>
            <Link to={`/dns-management/${id}`}>
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
              onConfirm={() => handleDelete(id)}
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

  const CreateDomain = () => {
    return (
      <React.Fragment>
        <Form form={form} layout="inline" onFinish={handleApply}>
          <Form.Item
            label="Domain name"
            name="domain_name"
            rules={[{ required: true, message: "Input domain name!" }]}
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
              className="button-update"
              size="large"
            >
              Create
            </Button>
          </Form.Item>
        </Form>
      </React.Fragment>
    );
  };

  return (
    <React.Fragment>
      <Content>
        <Row gutter={12}>
          <Col span={16}>
            <div className="card">
              <div className="container">
                <div className="container-header">
                  <h1>DNS Setting</h1>
                </div>
                <hr />
                <div className="dns-desc-container">
                  <CreateDomain />
                </div>
                <Form>
                  <div className="dns-desc-container">
                    <Form.Item>
                      <Table
                        columns={columns}
                        dataSource={datas}
                        pagination={false}
                        scroll={{ y: 450 }}
                      />
                    </Form.Item>
                  </div>
                </Form>
              </div>
            </div>
          </Col>
          <Col span={8}>
            <div className="card2">
              <div className="container">
                <div className="container-header">
                  <h1>Desciptions</h1>
                </div>
              </div>
            </div>
          </Col>
        </Row>
      </Content>
    </React.Fragment>
  );
};

export default DNSSetting;
